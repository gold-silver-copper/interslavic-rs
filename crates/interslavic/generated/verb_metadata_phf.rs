use phf::phf_map;
use super::VerbDictionaryEntry;

pub(crate) static VERB_METADATA: phf::Map<&'static str, &'static [VerbDictionaryEntry]> = phf_map! {
    "#doględati" => &[
        VerbDictionaryEntry { lemma: "#doględati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "#goniti" => &[
        VerbDictionaryEntry { lemma: "#goniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "#izdělati" => &[
        VerbDictionaryEntry { lemma: "#izdělati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "#izkljuvati" => &[
        VerbDictionaryEntry { lemma: "#izkljuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "#izobličati" => &[
        VerbDictionaryEntry { lemma: "#izobličati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "#izstųpiti" => &[
        VerbDictionaryEntry { lemma: "#izstųpiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "#oblěkati" => &[
        VerbDictionaryEntry { lemma: "#oblěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "#obstanoviti" => &[
        VerbDictionaryEntry { lemma: "#obstanoviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "#ocěnjati" => &[
        VerbDictionaryEntry { lemma: "#ocěnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "#odtiskati" => &[
        VerbDictionaryEntry { lemma: "#odtiskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "#orositi" => &[
        VerbDictionaryEntry { lemma: "#orositi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "#ovoščiti" => &[
        VerbDictionaryEntry { lemma: "#ovoščiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "#oznaniti" => &[
        VerbDictionaryEntry { lemma: "#oznaniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "#pnųti" => &[
        VerbDictionaryEntry { lemma: "#pnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "#posmotriti" => &[
        VerbDictionaryEntry { lemma: "#posmotriti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "#pretendovati" => &[
        VerbDictionaryEntry { lemma: "#pretendovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "#prědpostavjati" => &[
        VerbDictionaryEntry { lemma: "#prědpostavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "#prěznačati" => &[
        VerbDictionaryEntry { lemma: "#prěznačati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "#råzstlati" => &[
        VerbDictionaryEntry { lemma: "#råzstlati", addition: "(råzstelje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "#smotriti" => &[
        VerbDictionaryEntry { lemma: "#smotriti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "#spisati" => &[
        VerbDictionaryEntry { lemma: "#spisati", addition: "(spiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "#stvŕditi" => &[
        VerbDictionaryEntry { lemma: "#stvŕditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "#sypati" => &[
        VerbDictionaryEntry { lemma: "#sypati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "#sčityvati" => &[
        VerbDictionaryEntry { lemma: "#sčityvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "#sȯžegti" => &[
        VerbDictionaryEntry { lemma: "#sȯžegti", addition: "(sȯžže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "#urězati" => &[
        VerbDictionaryEntry { lemma: "#urězati", addition: "(urěže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "#učiniti" => &[
        VerbDictionaryEntry { lemma: "#učiniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "#vlězti" => &[
        VerbDictionaryEntry { lemma: "#vlězti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "#vsosati" => &[
        VerbDictionaryEntry { lemma: "#vsosati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "#vykati" => &[
        VerbDictionaryEntry { lemma: "#vykati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "#vytŕpěti" => &[
        VerbDictionaryEntry { lemma: "#vytŕpěti", addition: "(vytŕpi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "#vyznavati" => &[
        VerbDictionaryEntry { lemma: "#vyznavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "#vȯzdvigati" => &[
        VerbDictionaryEntry { lemma: "#vȯzdvigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "#vȯzkypyvati" => &[
        VerbDictionaryEntry { lemma: "#vȯzkypyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "#zakazati" => &[
        VerbDictionaryEntry { lemma: "#zakazati", addition: "(zakaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "#zasvětiti" => &[
        VerbDictionaryEntry { lemma: "#zasvětiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "abdikovati" => &[
        VerbDictionaryEntry { lemma: "abdikovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true },
    ],
    "abonovati" => &[
        VerbDictionaryEntry { lemma: "abonovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "absolutizovati" => &[
        VerbDictionaryEntry { lemma: "absolutizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "absorbovati" => &[
        VerbDictionaryEntry { lemma: "absorbovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "abstrahovati" => &[
        VerbDictionaryEntry { lemma: "abstrahovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "adaptovati" => &[
        VerbDictionaryEntry { lemma: "adaptovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "administrovati" => &[
        VerbDictionaryEntry { lemma: "administrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "adoptovati" => &[
        VerbDictionaryEntry { lemma: "adoptovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "adresovati" => &[
        VerbDictionaryEntry { lemma: "adresovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "agitovati" => &[
        VerbDictionaryEntry { lemma: "agitovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "agonizovati" => &[
        VerbDictionaryEntry { lemma: "agonizovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "akcentovati" => &[
        VerbDictionaryEntry { lemma: "akcentovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "akceptovati" => &[
        VerbDictionaryEntry { lemma: "akceptovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "aklimatizovati" => &[
        VerbDictionaryEntry { lemma: "aklimatizovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "akompanovati" => &[
        VerbDictionaryEntry { lemma: "akompanovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "aktivovati" => &[
        VerbDictionaryEntry { lemma: "aktivovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "aktualizovati" => &[
        VerbDictionaryEntry { lemma: "aktualizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "akumulovati" => &[
        VerbDictionaryEntry { lemma: "akumulovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "amnestovati" => &[
        VerbDictionaryEntry { lemma: "amnestovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "amortizovati" => &[
        VerbDictionaryEntry { lemma: "amortizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "amputovati" => &[
        VerbDictionaryEntry { lemma: "amputovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "analizovati" => &[
        VerbDictionaryEntry { lemma: "analizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "aneksovati" => &[
        VerbDictionaryEntry { lemma: "aneksovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "anulovati" => &[
        VerbDictionaryEntry { lemma: "anulovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "apelovati" => &[
        VerbDictionaryEntry { lemma: "apelovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "aplodovati" => &[
        VerbDictionaryEntry { lemma: "aplodovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "aranževati" => &[
        VerbDictionaryEntry { lemma: "aranževati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "areštovati" => &[
        VerbDictionaryEntry { lemma: "areštovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "argumentovati" => &[
        VerbDictionaryEntry { lemma: "argumentovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "arhivovati" => &[
        VerbDictionaryEntry { lemma: "arhivovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "asimilovati" => &[
        VerbDictionaryEntry { lemma: "asimilovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "atakovati" => &[
        VerbDictionaryEntry { lemma: "atakovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "avansovati" => &[
        VerbDictionaryEntry { lemma: "avansovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true },
    ],
    "avtomatizovati" => &[
        VerbDictionaryEntry { lemma: "avtomatizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "avtorizovati" => &[
        VerbDictionaryEntry { lemma: "avtorizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "avtostopovati" => &[
        VerbDictionaryEntry { lemma: "avtostopovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true },
    ],
    "bagatelizovati" => &[
        VerbDictionaryEntry { lemma: "bagatelizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "bajati" => &[
        VerbDictionaryEntry { lemma: "bajati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "balansovati" => &[
        VerbDictionaryEntry { lemma: "balansovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "balotovati" => &[
        VerbDictionaryEntry { lemma: "balotovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "balzamovati" => &[
        VerbDictionaryEntry { lemma: "balzamovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "barikadovati" => &[
        VerbDictionaryEntry { lemma: "barikadovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "barviti" => &[
        VerbDictionaryEntry { lemma: "barviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "baviti" => &[
        VerbDictionaryEntry { lemma: "baviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "bděti" => &[
        VerbDictionaryEntry { lemma: "bděti", addition: "(bdi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "besědovati" => &[
        VerbDictionaryEntry { lemma: "besědovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "bezpokojiti" => &[
        VerbDictionaryEntry { lemma: "bezpokojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "biti" => &[
        VerbDictionaryEntry { lemma: "biti", addition: "(bije)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "bičevati" => &[
        VerbDictionaryEntry { lemma: "bičevati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "blaznovati" => &[
        VerbDictionaryEntry { lemma: "blaznovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "blejati" => &[
        VerbDictionaryEntry { lemma: "blejati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "bljunųti" => &[
        VerbDictionaryEntry { lemma: "bljunųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "bljuvati" => &[
        VerbDictionaryEntry { lemma: "bljuvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "blokovati" => &[
        VerbDictionaryEntry { lemma: "blokovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "blågodariti" => &[
        VerbDictionaryEntry { lemma: "blågodariti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "blågoslavjati" => &[
        VerbDictionaryEntry { lemma: "blågoslavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "blågosloviti" => &[
        VerbDictionaryEntry { lemma: "blågosloviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "blågovolěti" => &[
        VerbDictionaryEntry { lemma: "blågovolěti", addition: "(blågovoli)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "blågoželati" => &[
        VerbDictionaryEntry { lemma: "blågoželati", addition: "(+3)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "blědněti" => &[
        VerbDictionaryEntry { lemma: "blědněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "blěskati" => &[
        VerbDictionaryEntry { lemma: "blěskati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "blěsknųti" => &[
        VerbDictionaryEntry { lemma: "blěsknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "blěstěti" => &[
        VerbDictionaryEntry { lemma: "blěstěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "blųditi" => &[
        VerbDictionaryEntry { lemma: "blųditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "blųkati" => &[
        VerbDictionaryEntry { lemma: "blųkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "bodati" => &[
        VerbDictionaryEntry { lemma: "bodati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "bodnųti" => &[
        VerbDictionaryEntry { lemma: "bodnųti", addition: "(bode)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "bogatěti" => &[
        VerbDictionaryEntry { lemma: "bogatěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "bogohuliti" => &[
        VerbDictionaryEntry { lemma: "bogohuliti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "bojevati" => &[
        VerbDictionaryEntry { lemma: "bojevati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "bojkotovati" => &[
        VerbDictionaryEntry { lemma: "bojkotovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "bolěti" => &[
        VerbDictionaryEntry { lemma: "bolěti", addition: "(bolěje)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "bolěti", addition: "(boli)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "bombardovati" => &[
        VerbDictionaryEntry { lemma: "bombardovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "bosti" => &[
        VerbDictionaryEntry { lemma: "bosti", addition: "(bode)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "brati" => &[
        VerbDictionaryEntry { lemma: "brati", addition: "(bere)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "breknųti" => &[
        VerbDictionaryEntry { lemma: "breknųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "brenknųti" => &[
        VerbDictionaryEntry { lemma: "brenknųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "brenčati" => &[
        VerbDictionaryEntry { lemma: "brenčati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "briti" => &[
        VerbDictionaryEntry { lemma: "briti", addition: "(brije)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "broditi" => &[
        VerbDictionaryEntry { lemma: "broditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "brusiti" => &[
        VerbDictionaryEntry { lemma: "brusiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "bryzgati" => &[
        VerbDictionaryEntry { lemma: "bryzgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "bryzgnųti" => &[
        VerbDictionaryEntry { lemma: "bryzgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "bråniti" => &[
        VerbDictionaryEntry { lemma: "bråniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "bråzditi" => &[
        VerbDictionaryEntry { lemma: "bråzditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "buditi" => &[
        VerbDictionaryEntry { lemma: "buditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "budovati" => &[
        VerbDictionaryEntry { lemma: "budovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "buhati" => &[
        VerbDictionaryEntry { lemma: "buhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "buhnųti" => &[
        VerbDictionaryEntry { lemma: "buhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "buriti" => &[
        VerbDictionaryEntry { lemma: "buriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "bučati" => &[
        VerbDictionaryEntry { lemma: "bučati", addition: "(buče)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "byti" => &[
        VerbDictionaryEntry { lemma: "byti", addition: "(jesm, jesi, jest, jesmo, jeste, sųt; byl; bųdų)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "byvati" => &[
        VerbDictionaryEntry { lemma: "byvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "bzdnųti" => &[
        VerbDictionaryEntry { lemma: "bzdnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "bzděti" => &[
        VerbDictionaryEntry { lemma: "bzděti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "běgati" => &[
        VerbDictionaryEntry { lemma: "běgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "běgti" => &[
        VerbDictionaryEntry { lemma: "běgti", addition: "(běži)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "běliti" => &[
        VerbDictionaryEntry { lemma: "běliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "bělěti" => &[
        VerbDictionaryEntry { lemma: "bělěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "cenzurovati" => &[
        VerbDictionaryEntry { lemma: "cenzurovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "citovati" => &[
        VerbDictionaryEntry { lemma: "citovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "cmokati" => &[
        VerbDictionaryEntry { lemma: "cmokati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "cviliti" => &[
        VerbDictionaryEntry { lemma: "cviliti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "cvěliti" => &[
        VerbDictionaryEntry { lemma: "cvěliti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "cvěsti" => &[
        VerbDictionaryEntry { lemma: "cvěsti", addition: "(cvěte)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "cvětati" => &[
        VerbDictionaryEntry { lemma: "cvětati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "cvětnųti" => &[
        VerbDictionaryEntry { lemma: "cvětnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "cvŕkati" => &[
        VerbDictionaryEntry { lemma: "cvŕkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "cěditi" => &[
        VerbDictionaryEntry { lemma: "cěditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "cěliti" => &[
        VerbDictionaryEntry { lemma: "cěliti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "cělovati" => &[
        VerbDictionaryEntry { lemma: "cělovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "cěniti" => &[
        VerbDictionaryEntry { lemma: "cěniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dariti" => &[
        VerbDictionaryEntry { lemma: "dariti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "darovati" => &[
        VerbDictionaryEntry { lemma: "darovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dati" => &[
        VerbDictionaryEntry { lemma: "dati", addition: "(da)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "datovati" => &[
        VerbDictionaryEntry { lemma: "datovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "daunlodovati" => &[
        VerbDictionaryEntry { lemma: "daunlodovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "davati" => &[
        VerbDictionaryEntry { lemma: "davati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "daviti" => &[
        VerbDictionaryEntry { lemma: "daviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dbati" => &[
        VerbDictionaryEntry { lemma: "dbati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "deaktivovati" => &[
        VerbDictionaryEntry { lemma: "deaktivovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "debatovati" => &[
        VerbDictionaryEntry { lemma: "debatovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "debelěti" => &[
        VerbDictionaryEntry { lemma: "debelěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "debjutovati" => &[
        VerbDictionaryEntry { lemma: "debjutovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true },
    ],
    "deblokovati" => &[
        VerbDictionaryEntry { lemma: "deblokovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true },
    ],
    "definiovati" => &[
        VerbDictionaryEntry { lemma: "definiovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "defisovati" => &[
        VerbDictionaryEntry { lemma: "defisovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "deformovati" => &[
        VerbDictionaryEntry { lemma: "deformovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "degenerovati" => &[
        VerbDictionaryEntry { lemma: "degenerovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true },
    ],
    "degradovati" => &[
        VerbDictionaryEntry { lemma: "degradovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "degustovati" => &[
        VerbDictionaryEntry { lemma: "degustovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "deklamovati" => &[
        VerbDictionaryEntry { lemma: "deklamovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "dekodovati" => &[
        VerbDictionaryEntry { lemma: "dekodovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "demonstrovati" => &[
        VerbDictionaryEntry { lemma: "demonstrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "demoralizovati" => &[
        VerbDictionaryEntry { lemma: "demoralizovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "deportovati" => &[
        VerbDictionaryEntry { lemma: "deportovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "deprimovati" => &[
        VerbDictionaryEntry { lemma: "deprimovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "destabilizovati" => &[
        VerbDictionaryEntry { lemma: "destabilizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "dezertovati" => &[
        VerbDictionaryEntry { lemma: "dezertovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true },
    ],
    "dezinfikovati" => &[
        VerbDictionaryEntry { lemma: "dezinfikovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "diktovati" => &[
        VerbDictionaryEntry { lemma: "diktovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "diskreditovati" => &[
        VerbDictionaryEntry { lemma: "diskreditovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "diskriminovati" => &[
        VerbDictionaryEntry { lemma: "diskriminovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true },
    ],
    "diskutovati" => &[
        VerbDictionaryEntry { lemma: "diskutovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "diskvalifikovati" => &[
        VerbDictionaryEntry { lemma: "diskvalifikovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "diviti" => &[
        VerbDictionaryEntry { lemma: "diviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dobaviti" => &[
        VerbDictionaryEntry { lemma: "dobaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dobavjati" => &[
        VerbDictionaryEntry { lemma: "dobavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dobrěti" => &[
        VerbDictionaryEntry { lemma: "dobrěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "dobyti" => &[
        VerbDictionaryEntry { lemma: "dobyti", addition: "(dobųde)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dobyvati" => &[
        VerbDictionaryEntry { lemma: "dobyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dodati" => &[
        VerbDictionaryEntry { lemma: "dodati", addition: "(doda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dodavati" => &[
        VerbDictionaryEntry { lemma: "dodavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "doganjati" => &[
        VerbDictionaryEntry { lemma: "doganjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dogarjati" => &[
        VerbDictionaryEntry { lemma: "dogarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "doględěti" => &[
        VerbDictionaryEntry { lemma: "doględěti", addition: "(doględi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dognati" => &[
        VerbDictionaryEntry { lemma: "dognati", addition: "(dogone)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dogorěti" => &[
        VerbDictionaryEntry { lemma: "dogorěti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dogovarjati" => &[
        VerbDictionaryEntry { lemma: "dogovarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dogovoriti" => &[
        VerbDictionaryEntry { lemma: "dogovoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dohoditi" => &[
        VerbDictionaryEntry { lemma: "dohoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "dojdti" => &[
        VerbDictionaryEntry { lemma: "dojdti", addition: "(dojde; došėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "dojehati" => &[
        VerbDictionaryEntry { lemma: "dojehati", addition: "(dojede)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "doježđati" => &[
        VerbDictionaryEntry { lemma: "doježđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "dojiti" => &[
        VerbDictionaryEntry { lemma: "dojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dokazati" => &[
        VerbDictionaryEntry { lemma: "dokazati", addition: "(dokaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dokazyvati" => &[
        VerbDictionaryEntry { lemma: "dokazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dokladati" => &[
        VerbDictionaryEntry { lemma: "dokladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dokonati" => &[
        VerbDictionaryEntry { lemma: "dokonati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dokonyvati" => &[
        VerbDictionaryEntry { lemma: "dokonyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dokončiti" => &[
        VerbDictionaryEntry { lemma: "dokončiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dokumentovati" => &[
        VerbDictionaryEntry { lemma: "dokumentovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "dokupiti" => &[
        VerbDictionaryEntry { lemma: "dokupiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dokupovati" => &[
        VerbDictionaryEntry { lemma: "dokupovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "doletěti" => &[
        VerbDictionaryEntry { lemma: "doletěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "doložiti" => &[
        VerbDictionaryEntry { lemma: "doložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dominovati" => &[
        VerbDictionaryEntry { lemma: "dominovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "domněvati" => &[
        VerbDictionaryEntry { lemma: "domněvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "donesti" => &[
        VerbDictionaryEntry { lemma: "donesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "donositi" => &[
        VerbDictionaryEntry { lemma: "donositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dopisati" => &[
        VerbDictionaryEntry { lemma: "dopisati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dopisyvati" => &[
        VerbDictionaryEntry { lemma: "dopisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "doplatiti" => &[
        VerbDictionaryEntry { lemma: "doplatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "doplaćati" => &[
        VerbDictionaryEntry { lemma: "doplaćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dopustiti" => &[
        VerbDictionaryEntry { lemma: "dopustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dopušćati" => &[
        VerbDictionaryEntry { lemma: "dopušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dopȯlniti" => &[
        VerbDictionaryEntry { lemma: "dopȯlniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dopȯlnjati" => &[
        VerbDictionaryEntry { lemma: "dopȯlnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dopȯlzati" => &[
        VerbDictionaryEntry { lemma: "dopȯlzati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "dopȯlzti" => &[
        VerbDictionaryEntry { lemma: "dopȯlzti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "dorastati" => &[
        VerbDictionaryEntry { lemma: "dorastati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "dorysovati" => &[
        VerbDictionaryEntry { lemma: "dorysovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dorysovyvati" => &[
        VerbDictionaryEntry { lemma: "dorysovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "doråsti" => &[
        VerbDictionaryEntry { lemma: "doråsti", addition: "(doråste)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "dorųčati" => &[
        VerbDictionaryEntry { lemma: "dorųčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dorųčiti" => &[
        VerbDictionaryEntry { lemma: "dorųčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dosaditi" => &[
        VerbDictionaryEntry { lemma: "dosaditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dosađati" => &[
        VerbDictionaryEntry { lemma: "dosađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "doskonaliti" => &[
        VerbDictionaryEntry { lemma: "doskonaliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dostati" => &[
        VerbDictionaryEntry { lemma: "dostati", addition: "(dostane)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dostavati" => &[
        VerbDictionaryEntry { lemma: "dostavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dostaviti" => &[
        VerbDictionaryEntry { lemma: "dostaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dostavjati" => &[
        VerbDictionaryEntry { lemma: "dostavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dostigati" => &[
        VerbDictionaryEntry { lemma: "dostigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dostignųti" => &[
        VerbDictionaryEntry { lemma: "dostignųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dosęgati" => &[
        VerbDictionaryEntry { lemma: "dosęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dosęgnųti" => &[
        VerbDictionaryEntry { lemma: "dosęgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dotknųti" => &[
        VerbDictionaryEntry { lemma: "dotknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dotykati" => &[
        VerbDictionaryEntry { lemma: "dotykati", addition: "(dotyče)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dovezti" => &[
        VerbDictionaryEntry { lemma: "dovezti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dovoziti" => &[
        VerbDictionaryEntry { lemma: "dovoziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dověriti" => &[
        VerbDictionaryEntry { lemma: "dověriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dověrjati" => &[
        VerbDictionaryEntry { lemma: "dověrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "dověrjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "dovŕšati" => &[
        VerbDictionaryEntry { lemma: "dovŕšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dovŕšiti" => &[
        VerbDictionaryEntry { lemma: "dovŕšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dozovati" => &[
        VerbDictionaryEntry { lemma: "dozovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dozrěti" => &[
        VerbDictionaryEntry { lemma: "dozrěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "dozrěvati" => &[
        VerbDictionaryEntry { lemma: "dozrěvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "dozvaljati" => &[
        VerbDictionaryEntry { lemma: "dozvaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dozvoliti" => &[
        VerbDictionaryEntry { lemma: "dozvoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dožiti" => &[
        VerbDictionaryEntry { lemma: "dožiti", addition: "(dožive)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "doživati" => &[
        VerbDictionaryEntry { lemma: "doživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "drapati" => &[
        VerbDictionaryEntry { lemma: "drapati", addition: "(drape)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "drapnųti" => &[
        VerbDictionaryEntry { lemma: "drapnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dražniti" => &[
        VerbDictionaryEntry { lemma: "dražniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "drgati" => &[
        VerbDictionaryEntry { lemma: "drgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "drgnųti" => &[
        VerbDictionaryEntry { lemma: "drgnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "drobiti" => &[
        VerbDictionaryEntry { lemma: "drobiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "drěmati" => &[
        VerbDictionaryEntry { lemma: "drěmati", addition: "(drěme)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "drěmnųti" => &[
        VerbDictionaryEntry { lemma: "drěmnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "drěti" => &[
        VerbDictionaryEntry { lemma: "drěti", addition: "(dre)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "držati" => &[
        VerbDictionaryEntry { lemma: "držati", addition: "(drži)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "dubliti" => &[
        VerbDictionaryEntry { lemma: "dubliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dumati" => &[
        VerbDictionaryEntry { lemma: "dumati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "durěti" => &[
        VerbDictionaryEntry { lemma: "durěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "dušiti" => &[
        VerbDictionaryEntry { lemma: "dušiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dvigati" => &[
        VerbDictionaryEntry { lemma: "dvigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dvignųti" => &[
        VerbDictionaryEntry { lemma: "dvignųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "dvojiti" => &[
        VerbDictionaryEntry { lemma: "dvojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dyhati" => &[
        VerbDictionaryEntry { lemma: "dyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "dyhtěti" => &[
        VerbDictionaryEntry { lemma: "dyhtěti", addition: "(dyhti)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "dymiti" => &[
        VerbDictionaryEntry { lemma: "dymiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "dyšati" => &[
        VerbDictionaryEntry { lemma: "dyšati", addition: "(dyše)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "dękovati" => &[
        VerbDictionaryEntry { lemma: "dękovati", addition: "(+3)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "dějati" => &[
        VerbDictionaryEntry { lemma: "dějati", addition: "(děje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dělati" => &[
        VerbDictionaryEntry { lemma: "dělati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "děliti" => &[
        VerbDictionaryEntry { lemma: "děliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "děti" => &[
        VerbDictionaryEntry { lemma: "děti", addition: "(děje/děne)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dŕkati" => &[
        VerbDictionaryEntry { lemma: "dŕkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "dŕti" => &[
        VerbDictionaryEntry { lemma: "dŕti", addition: "(dre)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dŕzati" => &[
        VerbDictionaryEntry { lemma: "dŕzati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "dŕznųti" => &[
        VerbDictionaryEntry { lemma: "dŕznųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "dŕžati" => &[
        VerbDictionaryEntry { lemma: "dŕžati", addition: "(dŕži)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dųbiti" => &[
        VerbDictionaryEntry { lemma: "dųbiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dųnųti" => &[
        VerbDictionaryEntry { lemma: "dųnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "dųti" => &[
        VerbDictionaryEntry { lemma: "dųti", addition: "(dme)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dȯhnųti" => &[
        VerbDictionaryEntry { lemma: "dȯhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "dȯlbti" => &[
        VerbDictionaryEntry { lemma: "dȯlbti", addition: "(dȯlbe)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dȯlgočasiti" => &[
        VerbDictionaryEntry { lemma: "dȯlgočasiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dȯlžiti" => &[
        VerbDictionaryEntry { lemma: "dȯlžiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "dȯžditi" => &[
        VerbDictionaryEntry { lemma: "dȯžditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "egzistovati" => &[
        VerbDictionaryEntry { lemma: "egzistovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ejakulovati" => &[
        VerbDictionaryEntry { lemma: "ejakulovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "eksperimentovati" => &[
        VerbDictionaryEntry { lemma: "eksperimentovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "eksploatovati" => &[
        VerbDictionaryEntry { lemma: "eksploatovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "eksplodovati" => &[
        VerbDictionaryEntry { lemma: "eksplodovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "eksportovati" => &[
        VerbDictionaryEntry { lemma: "eksportovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ekstragovati" => &[
        VerbDictionaryEntry { lemma: "ekstragovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "eliminovati" => &[
        VerbDictionaryEntry { lemma: "eliminovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "emigrovati" => &[
        VerbDictionaryEntry { lemma: "emigrovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "entuziazmovati" => &[
        VerbDictionaryEntry { lemma: "entuziazmovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "eskalovati" => &[
        VerbDictionaryEntry { lemma: "eskalovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true },
    ],
    "eskortovati" => &[
        VerbDictionaryEntry { lemma: "eskortovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "evakuovati" => &[
        VerbDictionaryEntry { lemma: "evakuovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "fabrikovati" => &[
        VerbDictionaryEntry { lemma: "fabrikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "falsifikovati" => &[
        VerbDictionaryEntry { lemma: "falsifikovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "farbovati" => &[
        VerbDictionaryEntry { lemma: "farbovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "filmovati" => &[
        VerbDictionaryEntry { lemma: "filmovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "filtrovati" => &[
        VerbDictionaryEntry { lemma: "filtrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "finansovati" => &[
        VerbDictionaryEntry { lemma: "finansovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "flirtovati" => &[
        VerbDictionaryEntry { lemma: "flirtovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "formalizovati" => &[
        VerbDictionaryEntry { lemma: "formalizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "formovati" => &[
        VerbDictionaryEntry { lemma: "formovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "formulovati" => &[
        VerbDictionaryEntry { lemma: "formulovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "fotografovati" => &[
        VerbDictionaryEntry { lemma: "fotografovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "frustrovati" => &[
        VerbDictionaryEntry { lemma: "frustrovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "funkcionovati" => &[
        VerbDictionaryEntry { lemma: "funkcionovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "gadati" => &[
        VerbDictionaryEntry { lemma: "gadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "galopovati" => &[
        VerbDictionaryEntry { lemma: "galopovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true },
    ],
    "galvanizovati" => &[
        VerbDictionaryEntry { lemma: "galvanizovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true },
    ],
    "garantovati" => &[
        VerbDictionaryEntry { lemma: "garantovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "garnirovati" => &[
        VerbDictionaryEntry { lemma: "garnirovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "gasiti" => &[
        VerbDictionaryEntry { lemma: "gasiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "gasnųti" => &[
        VerbDictionaryEntry { lemma: "gasnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "gazovati" => &[
        VerbDictionaryEntry { lemma: "gazovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "gazovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "generalizovati" => &[
        VerbDictionaryEntry { lemma: "generalizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "generovati" => &[
        VerbDictionaryEntry { lemma: "generovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "gestikulovati" => &[
        VerbDictionaryEntry { lemma: "gestikulovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "gladiti" => &[
        VerbDictionaryEntry { lemma: "gladiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "globalizovati" => &[
        VerbDictionaryEntry { lemma: "globalizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "glodati" => &[
        VerbDictionaryEntry { lemma: "glodati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "gluhnųti" => &[
        VerbDictionaryEntry { lemma: "gluhnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "glupěti" => &[
        VerbDictionaryEntry { lemma: "glupěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "glušiti" => &[
        VerbDictionaryEntry { lemma: "glušiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "glådovati" => &[
        VerbDictionaryEntry { lemma: "glådovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "glåsiti" => &[
        VerbDictionaryEntry { lemma: "glåsiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "glåsovati" => &[
        VerbDictionaryEntry { lemma: "glåsovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ględati" => &[
        VerbDictionaryEntry { lemma: "ględati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ględnųti" => &[
        VerbDictionaryEntry { lemma: "ględnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "ględěti" => &[
        VerbDictionaryEntry { lemma: "ględěti", addition: "(ględi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "gnati" => &[
        VerbDictionaryEntry { lemma: "gnati", addition: "(gone)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "gnesti" => &[
        VerbDictionaryEntry { lemma: "gnesti", addition: "(gnete)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "gniti" => &[
        VerbDictionaryEntry { lemma: "gniti", addition: "(gnije)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "gnojiti" => &[
        VerbDictionaryEntry { lemma: "gnojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "gněvati" => &[
        VerbDictionaryEntry { lemma: "gněvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "gnųti" => &[
        VerbDictionaryEntry { lemma: "gnųti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "godovati" => &[
        VerbDictionaryEntry { lemma: "godovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "gojiti" => &[
        VerbDictionaryEntry { lemma: "gojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "goliti" => &[
        VerbDictionaryEntry { lemma: "goliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "gorčiti" => &[
        VerbDictionaryEntry { lemma: "gorčiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "gorěti" => &[
        VerbDictionaryEntry { lemma: "gorěti", addition: "(gori)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "goršiti" => &[
        VerbDictionaryEntry { lemma: "goršiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "gostiti" => &[
        VerbDictionaryEntry { lemma: "gostiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "gotoviti" => &[
        VerbDictionaryEntry { lemma: "gotoviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "govoriti" => &[
        VerbDictionaryEntry { lemma: "govoriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "goŕknųti" => &[
        VerbDictionaryEntry { lemma: "goŕknųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "grabiti" => &[
        VerbDictionaryEntry { lemma: "grabiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "grabiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "graničiti" => &[
        VerbDictionaryEntry { lemma: "graničiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "gravirovati" => &[
        VerbDictionaryEntry { lemma: "gravirovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "grditi" => &[
        VerbDictionaryEntry { lemma: "grditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "grebti" => &[
        VerbDictionaryEntry { lemma: "grebti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "grnųti" => &[
        VerbDictionaryEntry { lemma: "grnųti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "grohtati" => &[
        VerbDictionaryEntry { lemma: "grohtati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "gromaditi" => &[
        VerbDictionaryEntry { lemma: "gromaditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "groziti" => &[
        VerbDictionaryEntry { lemma: "groziti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "grupovati" => &[
        VerbDictionaryEntry { lemma: "grupovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "gryzati" => &[
        VerbDictionaryEntry { lemma: "gryzati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "gryzti" => &[
        VerbDictionaryEntry { lemma: "gryzti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "grėměti" => &[
        VerbDictionaryEntry { lemma: "grėměti", addition: "(grėmi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "gręznųti" => &[
        VerbDictionaryEntry { lemma: "gręznųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "grěti" => &[
        VerbDictionaryEntry { lemma: "grěti", addition: "(grěje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "grěšiti" => &[
        VerbDictionaryEntry { lemma: "grěšiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "gubiti" => &[
        VerbDictionaryEntry { lemma: "gubiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "gybati" => &[
        VerbDictionaryEntry { lemma: "gybati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "gybnųti" => &[
        VerbDictionaryEntry { lemma: "gybnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "gųstiti" => &[
        VerbDictionaryEntry { lemma: "gųstiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "gųstěti" => &[
        VerbDictionaryEntry { lemma: "gųstěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "gȯltati" => &[
        VerbDictionaryEntry { lemma: "gȯltati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "gȯltnųti" => &[
        VerbDictionaryEntry { lemma: "gȯltnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "halucinovati" => &[
        VerbDictionaryEntry { lemma: "halucinovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "harakterizovati" => &[
        VerbDictionaryEntry { lemma: "harakterizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "harmonizovati" => &[
        VerbDictionaryEntry { lemma: "harmonizovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "hipnotizovati" => &[
        VerbDictionaryEntry { lemma: "hipnotizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "hlipati" => &[
        VerbDictionaryEntry { lemma: "hlipati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "hlipnųti" => &[
        VerbDictionaryEntry { lemma: "hlipnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "hlåditi" => &[
        VerbDictionaryEntry { lemma: "hlåditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "hlåděti" => &[
        VerbDictionaryEntry { lemma: "hlåděti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "hmuriti" => &[
        VerbDictionaryEntry { lemma: "hmuriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "hoditi" => &[
        VerbDictionaryEntry { lemma: "hoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "homogenizovati" => &[
        VerbDictionaryEntry { lemma: "homogenizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "hotěti" => &[
        VerbDictionaryEntry { lemma: "hotěti", addition: "(hoće)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "hovati" => &[
        VerbDictionaryEntry { lemma: "hovati", addition: "(hovaje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "hrapati" => &[
        VerbDictionaryEntry { lemma: "hrapati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "hrapnųti" => &[
        VerbDictionaryEntry { lemma: "hrapnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "hromati" => &[
        VerbDictionaryEntry { lemma: "hromati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "hroměti" => &[
        VerbDictionaryEntry { lemma: "hroměti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "hrupati" => &[
        VerbDictionaryEntry { lemma: "hrupati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "hrustati" => &[
        VerbDictionaryEntry { lemma: "hrustati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "hråniti" => &[
        VerbDictionaryEntry { lemma: "hråniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "hrčati" => &[
        VerbDictionaryEntry { lemma: "hrčati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "htěti" => &[
        VerbDictionaryEntry { lemma: "htěti", addition: "(hće)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "hudnųti" => &[
        VerbDictionaryEntry { lemma: "hudnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "hvaliti" => &[
        VerbDictionaryEntry { lemma: "hvaliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "hvatati" => &[
        VerbDictionaryEntry { lemma: "hvatati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "hvorěti" => &[
        VerbDictionaryEntry { lemma: "hvorěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "hvějati" => &[
        VerbDictionaryEntry { lemma: "hvějati", addition: "(hvěje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "hybiti" => &[
        VerbDictionaryEntry { lemma: "hybiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "idealizovati" => &[
        VerbDictionaryEntry { lemma: "idealizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "identifikovati" => &[
        VerbDictionaryEntry { lemma: "identifikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "idti" => &[
        VerbDictionaryEntry { lemma: "idti", addition: "(ide; šėl)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ignorovati" => &[
        VerbDictionaryEntry { lemma: "ignorovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "igrati" => &[
        VerbDictionaryEntry { lemma: "igrati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ikati" => &[
        VerbDictionaryEntry { lemma: "ikati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ilustrovati" => &[
        VerbDictionaryEntry { lemma: "ilustrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "imati" => &[
        VerbDictionaryEntry { lemma: "imati", addition: "(imaje)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "imati", addition: "(imaje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "imenovati" => &[
        VerbDictionaryEntry { lemma: "imenovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "imigrovati" => &[
        VerbDictionaryEntry { lemma: "imigrovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "imitovati" => &[
        VerbDictionaryEntry { lemma: "imitovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "implantovati" => &[
        VerbDictionaryEntry { lemma: "implantovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "imponovati" => &[
        VerbDictionaryEntry { lemma: "imponovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "importovati" => &[
        VerbDictionaryEntry { lemma: "importovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "improvizovati" => &[
        VerbDictionaryEntry { lemma: "improvizovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "iměti" => &[
        VerbDictionaryEntry { lemma: "iměti", addition: "(imaje)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "iměti", addition: "(imaje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "industrializovati" => &[
        VerbDictionaryEntry { lemma: "industrializovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "informovati" => &[
        VerbDictionaryEntry { lemma: "informovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "inicializovati" => &[
        VerbDictionaryEntry { lemma: "inicializovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "inspirovati" => &[
        VerbDictionaryEntry { lemma: "inspirovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "instalovati" => &[
        VerbDictionaryEntry { lemma: "instalovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "integrovati" => &[
        VerbDictionaryEntry { lemma: "integrovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "interesovati" => &[
        VerbDictionaryEntry { lemma: "interesovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "internacionalizovati" => &[
        VerbDictionaryEntry { lemma: "internacionalizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "interpretovati" => &[
        VerbDictionaryEntry { lemma: "interpretovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "intrigovati" => &[
        VerbDictionaryEntry { lemma: "intrigovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "investovati" => &[
        VerbDictionaryEntry { lemma: "investovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ironizovati" => &[
        VerbDictionaryEntry { lemma: "ironizovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "iskati" => &[
        VerbDictionaryEntry { lemma: "iskati", addition: "(išče)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "iskriti" => &[
        VerbDictionaryEntry { lemma: "iskriti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "istnovati" => &[
        VerbDictionaryEntry { lemma: "istnovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "izbaviti" => &[
        VerbDictionaryEntry { lemma: "izbaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "izbaviti", addition: "(+2)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izbavjati" => &[
        VerbDictionaryEntry { lemma: "izbavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "izbavjati", addition: "(+2)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izbirati" => &[
        VerbDictionaryEntry { lemma: "izbirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izbiti" => &[
        VerbDictionaryEntry { lemma: "izbiti", addition: "(izbije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izbivati" => &[
        VerbDictionaryEntry { lemma: "izbivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izbljuvati" => &[
        VerbDictionaryEntry { lemma: "izbljuvati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izbombardovati" => &[
        VerbDictionaryEntry { lemma: "izbombardovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izbrati" => &[
        VerbDictionaryEntry { lemma: "izbrati", addition: "(izbere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izbudovati" => &[
        VerbDictionaryEntry { lemma: "izbudovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izbuhati" => &[
        VerbDictionaryEntry { lemma: "izbuhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "izbuhnųti" => &[
        VerbDictionaryEntry { lemma: "izbuhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izběgati" => &[
        VerbDictionaryEntry { lemma: "izběgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "izběgati", addition: "(+2)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "izběgti" => &[
        VerbDictionaryEntry { lemma: "izběgti", addition: "(izběži) (+2)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "izběgti", addition: "(izběži)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izcěliti" => &[
        VerbDictionaryEntry { lemma: "izcěliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izcěljati" => &[
        VerbDictionaryEntry { lemma: "izcěljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izdati" => &[
        VerbDictionaryEntry { lemma: "izdati", addition: "(izda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izdavati" => &[
        VerbDictionaryEntry { lemma: "izdavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izdojiti" => &[
        VerbDictionaryEntry { lemma: "izdojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izdumati" => &[
        VerbDictionaryEntry { lemma: "izdumati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izdȯlbti" => &[
        VerbDictionaryEntry { lemma: "izdȯlbti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izganjati" => &[
        VerbDictionaryEntry { lemma: "izganjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izgladiti" => &[
        VerbDictionaryEntry { lemma: "izgladiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izglašati" => &[
        VerbDictionaryEntry { lemma: "izglašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izglåsiti" => &[
        VerbDictionaryEntry { lemma: "izglåsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izględati" => &[
        VerbDictionaryEntry { lemma: "izględati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "izględnųti" => &[
        VerbDictionaryEntry { lemma: "izględnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izgnati" => &[
        VerbDictionaryEntry { lemma: "izgnati", addition: "(izgone)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izgniti" => &[
        VerbDictionaryEntry { lemma: "izgniti", addition: "(izgnije)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izgojiti" => &[
        VerbDictionaryEntry { lemma: "izgojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izgorěti" => &[
        VerbDictionaryEntry { lemma: "izgorěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izgovarjati" => &[
        VerbDictionaryEntry { lemma: "izgovarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izgovoriti" => &[
        VerbDictionaryEntry { lemma: "izgovoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izgubiti" => &[
        VerbDictionaryEntry { lemma: "izgubiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izgynųti" => &[
        VerbDictionaryEntry { lemma: "izgynųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izhoditi" => &[
        VerbDictionaryEntry { lemma: "izhoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "izigrati" => &[
        VerbDictionaryEntry { lemma: "izigrati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izigryvati" => &[
        VerbDictionaryEntry { lemma: "izigryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izimati" => &[
        VerbDictionaryEntry { lemma: "izimati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "iziskati" => &[
        VerbDictionaryEntry { lemma: "iziskati", addition: "(izišče)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "iziskyvati" => &[
        VerbDictionaryEntry { lemma: "iziskyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izjasniti" => &[
        VerbDictionaryEntry { lemma: "izjasniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izjasnjati" => &[
        VerbDictionaryEntry { lemma: "izjasnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izjaviti" => &[
        VerbDictionaryEntry { lemma: "izjaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izjavjati" => &[
        VerbDictionaryEntry { lemma: "izjavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izjehati" => &[
        VerbDictionaryEntry { lemma: "izjehati", addition: "(izjede)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izježđati" => &[
        VerbDictionaryEntry { lemma: "izježđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "izjęti" => &[
        VerbDictionaryEntry { lemma: "izjęti", addition: "(izȯjme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izkalkulovati" => &[
        VerbDictionaryEntry { lemma: "izkalkulovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izkladati" => &[
        VerbDictionaryEntry { lemma: "izkladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izključati" => &[
        VerbDictionaryEntry { lemma: "izključati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izključiti" => &[
        VerbDictionaryEntry { lemma: "izključiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izkopati" => &[
        VerbDictionaryEntry { lemma: "izkopati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izkopyvati" => &[
        VerbDictionaryEntry { lemma: "izkopyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izkoreniti" => &[
        VerbDictionaryEntry { lemma: "izkoreniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izkorenjati" => &[
        VerbDictionaryEntry { lemma: "izkorenjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izkoristati" => &[
        VerbDictionaryEntry { lemma: "izkoristati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izkoristiti" => &[
        VerbDictionaryEntry { lemma: "izkoristiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izkoristyvati" => &[
        VerbDictionaryEntry { lemma: "izkoristyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izkositi" => &[
        VerbDictionaryEntry { lemma: "izkositi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izkovati" => &[
        VerbDictionaryEntry { lemma: "izkovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izkovyvati" => &[
        VerbDictionaryEntry { lemma: "izkovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izkrikati" => &[
        VerbDictionaryEntry { lemma: "izkrikati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izkriknųti" => &[
        VerbDictionaryEntry { lemma: "izkriknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izkrviti" => &[
        VerbDictionaryEntry { lemma: "izkrviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izkupiti" => &[
        VerbDictionaryEntry { lemma: "izkupiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izkusiti" => &[
        VerbDictionaryEntry { lemma: "izkusiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izkušati" => &[
        VerbDictionaryEntry { lemma: "izkušati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izkydati" => &[
        VerbDictionaryEntry { lemma: "izkydati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izkydnųti" => &[
        VerbDictionaryEntry { lemma: "izkydnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izkųpati" => &[
        VerbDictionaryEntry { lemma: "izkųpati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izlagati" => &[
        VerbDictionaryEntry { lemma: "izlagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izliti" => &[
        VerbDictionaryEntry { lemma: "izliti", addition: "(izlije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izlivati" => &[
        VerbDictionaryEntry { lemma: "izlivati", addition: "(izlije)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izlomiti" => &[
        VerbDictionaryEntry { lemma: "izlomiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izloviti" => &[
        VerbDictionaryEntry { lemma: "izloviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izložiti" => &[
        VerbDictionaryEntry { lemma: "izložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izlęgti" => &[
        VerbDictionaryEntry { lemma: "izlęgti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izlězati" => &[
        VerbDictionaryEntry { lemma: "izlězati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "izlězti" => &[
        VerbDictionaryEntry { lemma: "izlězti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izlěčiti" => &[
        VerbDictionaryEntry { lemma: "izlěčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izmagati" => &[
        VerbDictionaryEntry { lemma: "izmagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izmamiti" => &[
        VerbDictionaryEntry { lemma: "izmamiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izmamjati" => &[
        VerbDictionaryEntry { lemma: "izmamjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izmesti" => &[
        VerbDictionaryEntry { lemma: "izmesti", addition: "(izmete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izmirati" => &[
        VerbDictionaryEntry { lemma: "izmirati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "izmodelovati" => &[
        VerbDictionaryEntry { lemma: "izmodelovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izmogti" => &[
        VerbDictionaryEntry { lemma: "izmogti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izmoriti" => &[
        VerbDictionaryEntry { lemma: "izmoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izmočiti" => &[
        VerbDictionaryEntry { lemma: "izmočiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izmrěti" => &[
        VerbDictionaryEntry { lemma: "izmrěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izmysliti" => &[
        VerbDictionaryEntry { lemma: "izmysliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izmysljati" => &[
        VerbDictionaryEntry { lemma: "izmysljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izměniti" => &[
        VerbDictionaryEntry { lemma: "izměniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izměnjati" => &[
        VerbDictionaryEntry { lemma: "izměnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izměriti" => &[
        VerbDictionaryEntry { lemma: "izměriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izměstiti" => &[
        VerbDictionaryEntry { lemma: "izměstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izmětati" => &[
        VerbDictionaryEntry { lemma: "izmětati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izměšćati" => &[
        VerbDictionaryEntry { lemma: "izměšćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "iznahoditi" => &[
        VerbDictionaryEntry { lemma: "iznahoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "iznajdti" => &[
        VerbDictionaryEntry { lemma: "iznajdti", addition: "(iznajde; iznašėl)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "iznajmati" => &[
        VerbDictionaryEntry { lemma: "iznajmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "iznajęti" => &[
        VerbDictionaryEntry { lemma: "iznajęti", addition: "(iznajme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "iznasilovati" => &[
        VerbDictionaryEntry { lemma: "iznasilovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "iznemogti" => &[
        VerbDictionaryEntry { lemma: "iznemogti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "iznesti" => &[
        VerbDictionaryEntry { lemma: "iznesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izniknųti" => &[
        VerbDictionaryEntry { lemma: "izniknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izniščiti" => &[
        VerbDictionaryEntry { lemma: "izniščiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "iznositi" => &[
        VerbDictionaryEntry { lemma: "iznositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "iznuditi" => &[
        VerbDictionaryEntry { lemma: "iznuditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "iznuriti" => &[
        VerbDictionaryEntry { lemma: "iznuriti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "iznurjati" => &[
        VerbDictionaryEntry { lemma: "iznurjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "iznuđati" => &[
        VerbDictionaryEntry { lemma: "iznuđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izobličiti" => &[
        VerbDictionaryEntry { lemma: "izobličiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izobraziti" => &[
        VerbDictionaryEntry { lemma: "izobraziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izobražati" => &[
        VerbDictionaryEntry { lemma: "izobražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izolovati" => &[
        VerbDictionaryEntry { lemma: "izolovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izopačati" => &[
        VerbDictionaryEntry { lemma: "izopačati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izopačiti" => &[
        VerbDictionaryEntry { lemma: "izopačiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izorati" => &[
        VerbDictionaryEntry { lemma: "izorati", addition: "(izoŕe)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izpadati" => &[
        VerbDictionaryEntry { lemma: "izpadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "izpasti" => &[
        VerbDictionaryEntry { lemma: "izpasti", addition: "(izpade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izpekti" => &[
        VerbDictionaryEntry { lemma: "izpekti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izpepeliti" => &[
        VerbDictionaryEntry { lemma: "izpepeliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izphati" => &[
        VerbDictionaryEntry { lemma: "izphati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izpihati" => &[
        VerbDictionaryEntry { lemma: "izpihati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izpiti" => &[
        VerbDictionaryEntry { lemma: "izpiti", addition: "(izpije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izplatiti" => &[
        VerbDictionaryEntry { lemma: "izplatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izplaćati" => &[
        VerbDictionaryEntry { lemma: "izplaćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izpljunųti" => &[
        VerbDictionaryEntry { lemma: "izpljunųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izpljuvati" => &[
        VerbDictionaryEntry { lemma: "izpljuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izpluti" => &[
        VerbDictionaryEntry { lemma: "izpluti", addition: "(izplove)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izplyvati" => &[
        VerbDictionaryEntry { lemma: "izplyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "izplyvti" => &[
        VerbDictionaryEntry { lemma: "izplyvti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izplåšiti" => &[
        VerbDictionaryEntry { lemma: "izplåšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izplěti" => &[
        VerbDictionaryEntry { lemma: "izplěti", addition: "(izplěje/izplěve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izpovědati" => &[
        VerbDictionaryEntry { lemma: "izpovědati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izpověděti" => &[
        VerbDictionaryEntry { lemma: "izpověděti", addition: "(izpově)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izprati" => &[
        VerbDictionaryEntry { lemma: "izprati", addition: "(izpere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izpraviti" => &[
        VerbDictionaryEntry { lemma: "izpraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izpravjati" => &[
        VerbDictionaryEntry { lemma: "izpravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izprašati" => &[
        VerbDictionaryEntry { lemma: "izprašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izprobovati" => &[
        VerbDictionaryEntry { lemma: "izprobovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izprobovyvati" => &[
        VerbDictionaryEntry { lemma: "izprobovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izprositi" => &[
        VerbDictionaryEntry { lemma: "izprositi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izpråzdniti" => &[
        VerbDictionaryEntry { lemma: "izpråzdniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izpråzdnjati" => &[
        VerbDictionaryEntry { lemma: "izpråzdnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izpręgati" => &[
        VerbDictionaryEntry { lemma: "izpręgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izpręgti" => &[
        VerbDictionaryEntry { lemma: "izpręgti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izpustiti" => &[
        VerbDictionaryEntry { lemma: "izpustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izpušćati" => &[
        VerbDictionaryEntry { lemma: "izpušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izpytati" => &[
        VerbDictionaryEntry { lemma: "izpytati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izpytyvati" => &[
        VerbDictionaryEntry { lemma: "izpytyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izpųditi" => &[
        VerbDictionaryEntry { lemma: "izpųditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izpȯlniti" => &[
        VerbDictionaryEntry { lemma: "izpȯlniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izpȯlnjati" => &[
        VerbDictionaryEntry { lemma: "izpȯlnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izpȯlzati" => &[
        VerbDictionaryEntry { lemma: "izpȯlzati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "izpȯlzti" => &[
        VerbDictionaryEntry { lemma: "izpȯlzti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izravnjati" => &[
        VerbDictionaryEntry { lemma: "izravnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izraziti" => &[
        VerbDictionaryEntry { lemma: "izraziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izražati" => &[
        VerbDictionaryEntry { lemma: "izražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izrvati" => &[
        VerbDictionaryEntry { lemma: "izrvati", addition: "(izrve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izryvati" => &[
        VerbDictionaryEntry { lemma: "izryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izråbiti" => &[
        VerbDictionaryEntry { lemma: "izråbiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izråbotati" => &[
        VerbDictionaryEntry { lemma: "izråbotati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izråbotyvati" => &[
        VerbDictionaryEntry { lemma: "izråbotyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izråsti" => &[
        VerbDictionaryEntry { lemma: "izråsti", addition: "(izråste)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izråvniti" => &[
        VerbDictionaryEntry { lemma: "izråvniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izrěkati" => &[
        VerbDictionaryEntry { lemma: "izrěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izrěkti" => &[
        VerbDictionaryEntry { lemma: "izrěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izrězati" => &[
        VerbDictionaryEntry { lemma: "izrězati", addition: "(izrěže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izrězyvati" => &[
        VerbDictionaryEntry { lemma: "izrězyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izrųbati" => &[
        VerbDictionaryEntry { lemma: "izrųbati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izrųbyvati" => &[
        VerbDictionaryEntry { lemma: "izrųbyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izseliti" => &[
        VerbDictionaryEntry { lemma: "izseliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izseljati" => &[
        VerbDictionaryEntry { lemma: "izseljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izslati" => &[
        VerbDictionaryEntry { lemma: "izslati", addition: "(izšlje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izslavjati" => &[
        VerbDictionaryEntry { lemma: "izslavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izsloviti" => &[
        VerbDictionaryEntry { lemma: "izsloviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izslušati" => &[
        VerbDictionaryEntry { lemma: "izslušati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izslušivati" => &[
        VerbDictionaryEntry { lemma: "izslušivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izslědovati" => &[
        VerbDictionaryEntry { lemma: "izslědovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izsmějati" => &[
        VerbDictionaryEntry { lemma: "izsmějati", addition: "(izsměje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izsmějivati" => &[
        VerbDictionaryEntry { lemma: "izsmějivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izstaviti" => &[
        VerbDictionaryEntry { lemma: "izstaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izstavjati" => &[
        VerbDictionaryEntry { lemma: "izstavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izstrěliti" => &[
        VerbDictionaryEntry { lemma: "izstrěliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izstųpati" => &[
        VerbDictionaryEntry { lemma: "izstųpati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "izstųpiti" => &[
        VerbDictionaryEntry { lemma: "izstųpiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izsunųti" => &[
        VerbDictionaryEntry { lemma: "izsunųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izsuvati" => &[
        VerbDictionaryEntry { lemma: "izsuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izsušati" => &[
        VerbDictionaryEntry { lemma: "izsušati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izsušiti" => &[
        VerbDictionaryEntry { lemma: "izsušiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izsyhati" => &[
        VerbDictionaryEntry { lemma: "izsyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "izsylati" => &[
        VerbDictionaryEntry { lemma: "izsylati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izsypati" => &[
        VerbDictionaryEntry { lemma: "izsypati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izsysati" => &[
        VerbDictionaryEntry { lemma: "izsysati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izsěkati" => &[
        VerbDictionaryEntry { lemma: "izsěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izsěkti" => &[
        VerbDictionaryEntry { lemma: "izsěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izsȯhnųti" => &[
        VerbDictionaryEntry { lemma: "izsȯhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izsȯsati" => &[
        VerbDictionaryEntry { lemma: "izsȯsati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "iztekti" => &[
        VerbDictionaryEntry { lemma: "iztekti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "iztirati" => &[
        VerbDictionaryEntry { lemma: "iztirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "iztkati" => &[
        VerbDictionaryEntry { lemma: "iztkati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "iztratiti" => &[
        VerbDictionaryEntry { lemma: "iztratiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "iztrgati" => &[
        VerbDictionaryEntry { lemma: "iztrgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "iztrgnųti" => &[
        VerbDictionaryEntry { lemma: "iztrgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "iztręsati" => &[
        VerbDictionaryEntry { lemma: "iztręsati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "iztręsti" => &[
        VerbDictionaryEntry { lemma: "iztręsti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "iztrěti" => &[
        VerbDictionaryEntry { lemma: "iztrěti", addition: "(iztre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "iztvarjati" => &[
        VerbDictionaryEntry { lemma: "iztvarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "iztvoriti" => &[
        VerbDictionaryEntry { lemma: "iztvoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "iztęgati" => &[
        VerbDictionaryEntry { lemma: "iztęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "iztęgnųti" => &[
        VerbDictionaryEntry { lemma: "iztęgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "iztěkati" => &[
        VerbDictionaryEntry { lemma: "iztěkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "iztŕpěti" => &[
        VerbDictionaryEntry { lemma: "iztŕpěti", addition: "(iztŕpi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "iztŕti" => &[
        VerbDictionaryEntry { lemma: "iztŕti", addition: "(iztre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "iztȯlkti" => &[
        VerbDictionaryEntry { lemma: "iztȯlkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izumirati" => &[
        VerbDictionaryEntry { lemma: "izumirati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "izumrěti" => &[
        VerbDictionaryEntry { lemma: "izumrěti", addition: "(izumre)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izumŕti" => &[
        VerbDictionaryEntry { lemma: "izumŕti", addition: "(izumre)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izučati" => &[
        VerbDictionaryEntry { lemma: "izučati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izučiti" => &[
        VerbDictionaryEntry { lemma: "izučiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izvajati" => &[
        VerbDictionaryEntry { lemma: "izvajati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izvaljnjati" => &[
        VerbDictionaryEntry { lemma: "izvaljnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izvesti" => &[
        VerbDictionaryEntry { lemma: "izvesti", addition: "(izvede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izvezti" => &[
        VerbDictionaryEntry { lemma: "izvezti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izvihnųti" => &[
        VerbDictionaryEntry { lemma: "izvihnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izviniti" => &[
        VerbDictionaryEntry { lemma: "izviniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izvinjati" => &[
        VerbDictionaryEntry { lemma: "izvinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izvlastniti" => &[
        VerbDictionaryEntry { lemma: "izvlastniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izvlastnjati" => &[
        VerbDictionaryEntry { lemma: "izvlastnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izvlěkati" => &[
        VerbDictionaryEntry { lemma: "izvlěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izvlěkti" => &[
        VerbDictionaryEntry { lemma: "izvlěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izvoditi" => &[
        VerbDictionaryEntry { lemma: "izvoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izvoliti" => &[
        VerbDictionaryEntry { lemma: "izvoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izvoljniti" => &[
        VerbDictionaryEntry { lemma: "izvoljniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izvoziti" => &[
        VerbDictionaryEntry { lemma: "izvoziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izvraćati" => &[
        VerbDictionaryEntry { lemma: "izvraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izvråtiti" => &[
        VerbDictionaryEntry { lemma: "izvråtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izvěstiti" => &[
        VerbDictionaryEntry { lemma: "izvěstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izvěšćati" => &[
        VerbDictionaryEntry { lemma: "izvěšćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izznačati" => &[
        VerbDictionaryEntry { lemma: "izznačati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izznačiti" => &[
        VerbDictionaryEntry { lemma: "izznačiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izzvati" => &[
        VerbDictionaryEntry { lemma: "izzvati", addition: "(izzȯve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izzyvati" => &[
        VerbDictionaryEntry { lemma: "izzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izčezati" => &[
        VerbDictionaryEntry { lemma: "izčezati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "izčeznųti" => &[
        VerbDictionaryEntry { lemma: "izčeznųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "izčisliti" => &[
        VerbDictionaryEntry { lemma: "izčisliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izčisljati" => &[
        VerbDictionaryEntry { lemma: "izčisljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izčistiti" => &[
        VerbDictionaryEntry { lemma: "izčistiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izčrkati" => &[
        VerbDictionaryEntry { lemma: "izčrkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izčrknųti" => &[
        VerbDictionaryEntry { lemma: "izčrknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izčrpati" => &[
        VerbDictionaryEntry { lemma: "izčrpati", addition: "(izčrpe)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izčrpyvati" => &[
        VerbDictionaryEntry { lemma: "izčrpyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izšiti" => &[
        VerbDictionaryEntry { lemma: "izšiti", addition: "(izšije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izšivati" => &[
        VerbDictionaryEntry { lemma: "izšivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "izškoliti" => &[
        VerbDictionaryEntry { lemma: "izškoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izžęti" => &[
        VerbDictionaryEntry { lemma: "izžęti", addition: "(izžne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "izȯjdti" => &[
        VerbDictionaryEntry { lemma: "izȯjdti", addition: "(izȯjde; izšėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "jalověti" => &[
        VerbDictionaryEntry { lemma: "jalověti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "jasněti" => &[
        VerbDictionaryEntry { lemma: "jasněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "je" => &[
        VerbDictionaryEntry { lemma: "je", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "jebati" => &[
        VerbDictionaryEntry { lemma: "jebati", addition: "(jebe)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "jedati" => &[
        VerbDictionaryEntry { lemma: "jedati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "jehati" => &[
        VerbDictionaryEntry { lemma: "jehati", addition: "(jede)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "jest" => &[
        VerbDictionaryEntry { lemma: "jest", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "jesti" => &[
        VerbDictionaryEntry { lemma: "jesti", addition: "(je)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "jestvovati" => &[
        VerbDictionaryEntry { lemma: "jestvovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "jezditi" => &[
        VerbDictionaryEntry { lemma: "jezditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ježiti" => &[
        VerbDictionaryEntry { lemma: "ježiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "jęti" => &[
        VerbDictionaryEntry { lemma: "jęti", addition: "(jme)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "jęčati" => &[
        VerbDictionaryEntry { lemma: "jęčati", addition: "(jęči)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kaditi" => &[
        VerbDictionaryEntry { lemma: "kaditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kakati" => &[
        VerbDictionaryEntry { lemma: "kakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kaliti" => &[
        VerbDictionaryEntry { lemma: "kaliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kalkulovati" => &[
        VerbDictionaryEntry { lemma: "kalkulovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kamenovati" => &[
        VerbDictionaryEntry { lemma: "kamenovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kameněti" => &[
        VerbDictionaryEntry { lemma: "kameněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kanalizovati" => &[
        VerbDictionaryEntry { lemma: "kanalizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kandidovati" => &[
        VerbDictionaryEntry { lemma: "kandidovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kanonizovati" => &[
        VerbDictionaryEntry { lemma: "kanonizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kapati" => &[
        VerbDictionaryEntry { lemma: "kapati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kapitulovati" => &[
        VerbDictionaryEntry { lemma: "kapitulovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kapnųti" => &[
        VerbDictionaryEntry { lemma: "kapnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "kaprizovati" => &[
        VerbDictionaryEntry { lemma: "kaprizovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "karati" => &[
        VerbDictionaryEntry { lemma: "karati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kastrovati" => &[
        VerbDictionaryEntry { lemma: "kastrovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "kategorizovati" => &[
        VerbDictionaryEntry { lemma: "kategorizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "kazati" => &[
        VerbDictionaryEntry { lemma: "kazati", addition: "(kaže)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kaziti" => &[
        VerbDictionaryEntry { lemma: "kaziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kazniti" => &[
        VerbDictionaryEntry { lemma: "kazniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kašljati" => &[
        VerbDictionaryEntry { lemma: "kašljati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "klasifikovati" => &[
        VerbDictionaryEntry { lemma: "klasifikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "klasti" => &[
        VerbDictionaryEntry { lemma: "klasti", addition: "(klade)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "klejiti" => &[
        VerbDictionaryEntry { lemma: "klejiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "klevetati" => &[
        VerbDictionaryEntry { lemma: "klevetati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "klicati" => &[
        VerbDictionaryEntry { lemma: "klicati", addition: "(kliče)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "klikati" => &[
        VerbDictionaryEntry { lemma: "klikati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kliknųti" => &[
        VerbDictionaryEntry { lemma: "kliknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "kljunųti" => &[
        VerbDictionaryEntry { lemma: "kljunųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "kljusati" => &[
        VerbDictionaryEntry { lemma: "kljusati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kljuvati" => &[
        VerbDictionaryEntry { lemma: "kljuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "klokotati" => &[
        VerbDictionaryEntry { lemma: "klokotati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kloniti" => &[
        VerbDictionaryEntry { lemma: "kloniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "klåti" => &[
        VerbDictionaryEntry { lemma: "klåti", addition: "(kolje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "klåtiti" => &[
        VerbDictionaryEntry { lemma: "klåtiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "klęknųti" => &[
        VerbDictionaryEntry { lemma: "klęknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "klęti" => &[
        VerbDictionaryEntry { lemma: "klęti", addition: "(klne)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "klęčati" => &[
        VerbDictionaryEntry { lemma: "klęčati", addition: "(klęče)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "klěskati" => &[
        VerbDictionaryEntry { lemma: "klěskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kodifikovati" => &[
        VerbDictionaryEntry { lemma: "kodifikovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "kodovati" => &[
        VerbDictionaryEntry { lemma: "kodovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "koketovati" => &[
        VerbDictionaryEntry { lemma: "koketovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kokodakati" => &[
        VerbDictionaryEntry { lemma: "kokodakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kolonizovati" => &[
        VerbDictionaryEntry { lemma: "kolonizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "kolorovati" => &[
        VerbDictionaryEntry { lemma: "kolorovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kolovati" => &[
        VerbDictionaryEntry { lemma: "kolovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kolędovati" => &[
        VerbDictionaryEntry { lemma: "kolędovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kolěbati" => &[
        VerbDictionaryEntry { lemma: "kolěbati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kombinovati" => &[
        VerbDictionaryEntry { lemma: "kombinovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "komentovati" => &[
        VerbDictionaryEntry { lemma: "komentovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kompensovati" => &[
        VerbDictionaryEntry { lemma: "kompensovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "kompjuterizovati" => &[
        VerbDictionaryEntry { lemma: "kompjuterizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "komplikovati" => &[
        VerbDictionaryEntry { lemma: "komplikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "komponovati" => &[
        VerbDictionaryEntry { lemma: "komponovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "komunikovati" => &[
        VerbDictionaryEntry { lemma: "komunikovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "koncentrovati" => &[
        VerbDictionaryEntry { lemma: "koncentrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "konfiskovati" => &[
        VerbDictionaryEntry { lemma: "konfiskovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "konfliktovati" => &[
        VerbDictionaryEntry { lemma: "konfliktovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "konkurovati" => &[
        VerbDictionaryEntry { lemma: "konkurovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "konservovati" => &[
        VerbDictionaryEntry { lemma: "konservovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "konstruovati" => &[
        VerbDictionaryEntry { lemma: "konstruovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kontrabandovati" => &[
        VerbDictionaryEntry { lemma: "kontrabandovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kontrolovati" => &[
        VerbDictionaryEntry { lemma: "kontrolovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "končati" => &[
        VerbDictionaryEntry { lemma: "končati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kooperovati" => &[
        VerbDictionaryEntry { lemma: "kooperovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "koordinovati" => &[
        VerbDictionaryEntry { lemma: "koordinovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "kopati" => &[
        VerbDictionaryEntry { lemma: "kopati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kopijovati" => &[
        VerbDictionaryEntry { lemma: "kopijovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kopirovati" => &[
        VerbDictionaryEntry { lemma: "kopirovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "korelovati" => &[
        VerbDictionaryEntry { lemma: "korelovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "korigovati" => &[
        VerbDictionaryEntry { lemma: "korigovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "koristati" => &[
        VerbDictionaryEntry { lemma: "koristati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "koristiti" => &[
        VerbDictionaryEntry { lemma: "koristiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "koriti" => &[
        VerbDictionaryEntry { lemma: "koriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "koronovati" => &[
        VerbDictionaryEntry { lemma: "koronovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kositi" => &[
        VerbDictionaryEntry { lemma: "kositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kovati" => &[
        VerbDictionaryEntry { lemma: "kovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "koštovati" => &[
        VerbDictionaryEntry { lemma: "koštovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "krakati" => &[
        VerbDictionaryEntry { lemma: "krakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "krasiti" => &[
        VerbDictionaryEntry { lemma: "krasiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "krasti" => &[
        VerbDictionaryEntry { lemma: "krasti", addition: "(krade)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kresati" => &[
        VerbDictionaryEntry { lemma: "kresati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "krijumčariti" => &[
        VerbDictionaryEntry { lemma: "krijumčariti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kriknųti" => &[
        VerbDictionaryEntry { lemma: "kriknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "kristalizovati" => &[
        VerbDictionaryEntry { lemma: "kristalizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "kritikovati" => &[
        VerbDictionaryEntry { lemma: "kritikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "krivditi" => &[
        VerbDictionaryEntry { lemma: "krivditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kriviti" => &[
        VerbDictionaryEntry { lemma: "kriviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kričati" => &[
        VerbDictionaryEntry { lemma: "kričati", addition: "(kriči)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "krjakati" => &[
        VerbDictionaryEntry { lemma: "krjakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "krmiti" => &[
        VerbDictionaryEntry { lemma: "krmiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "krojiti" => &[
        VerbDictionaryEntry { lemma: "krojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kropiti" => &[
        VerbDictionaryEntry { lemma: "kropiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "krotiti" => &[
        VerbDictionaryEntry { lemma: "krotiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "krvaviti" => &[
        VerbDictionaryEntry { lemma: "krvaviti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kryti" => &[
        VerbDictionaryEntry { lemma: "kryti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "krčiti" => &[
        VerbDictionaryEntry { lemma: "krčiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "krėstiti" => &[
        VerbDictionaryEntry { lemma: "krėstiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "krěpiti" => &[
        VerbDictionaryEntry { lemma: "krěpiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "krěpěti" => &[
        VerbDictionaryEntry { lemma: "krěpěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "krųtiti" => &[
        VerbDictionaryEntry { lemma: "krųtiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "krųžiti" => &[
        VerbDictionaryEntry { lemma: "krųžiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "krȯšiti" => &[
        VerbDictionaryEntry { lemma: "krȯšiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kuhati" => &[
        VerbDictionaryEntry { lemma: "kuhati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kukati" => &[
        VerbDictionaryEntry { lemma: "kukati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kuljgati" => &[
        VerbDictionaryEntry { lemma: "kuljgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kulminovati" => &[
        VerbDictionaryEntry { lemma: "kulminovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kupiti" => &[
        VerbDictionaryEntry { lemma: "kupiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "kupovati" => &[
        VerbDictionaryEntry { lemma: "kupovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kuriti" => &[
        VerbDictionaryEntry { lemma: "kuriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kustomizovati" => &[
        VerbDictionaryEntry { lemma: "kustomizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "kvakati" => &[
        VerbDictionaryEntry { lemma: "kvakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kvalifikovati" => &[
        VerbDictionaryEntry { lemma: "kvalifikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kvasiti" => &[
        VerbDictionaryEntry { lemma: "kvasiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kvičati" => &[
        VerbDictionaryEntry { lemma: "kvičati", addition: "(kviče)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kvokati" => &[
        VerbDictionaryEntry { lemma: "kvokati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kydati" => &[
        VerbDictionaryEntry { lemma: "kydati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kydnųti" => &[
        VerbDictionaryEntry { lemma: "kydnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "kyhati" => &[
        VerbDictionaryEntry { lemma: "kyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kyhnųti" => &[
        VerbDictionaryEntry { lemma: "kyhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "kymati" => &[
        VerbDictionaryEntry { lemma: "kymati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kymnųti" => &[
        VerbDictionaryEntry { lemma: "kymnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "kypěti" => &[
        VerbDictionaryEntry { lemma: "kypěti", addition: "(kipi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kysnųti" => &[
        VerbDictionaryEntry { lemma: "kysnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kyvati" => &[
        VerbDictionaryEntry { lemma: "kyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "kyvnųti" => &[
        VerbDictionaryEntry { lemma: "kyvnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "kųpati" => &[
        VerbDictionaryEntry { lemma: "kųpati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kųsati" => &[
        VerbDictionaryEntry { lemma: "kųsati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "kųsnųti" => &[
        VerbDictionaryEntry { lemma: "kųsnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "lajati" => &[
        VerbDictionaryEntry { lemma: "lajati", addition: "(laje)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "lakovati" => &[
        VerbDictionaryEntry { lemma: "lakovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "lamati" => &[
        VerbDictionaryEntry { lemma: "lamati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "lapati" => &[
        VerbDictionaryEntry { lemma: "lapati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "laskati" => &[
        VerbDictionaryEntry { lemma: "laskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "latati" => &[
        VerbDictionaryEntry { lemma: "latati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "laziti" => &[
        VerbDictionaryEntry { lemma: "laziti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "leděniti" => &[
        VerbDictionaryEntry { lemma: "leděniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "leděněti" => &[
        VerbDictionaryEntry { lemma: "leděněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "legalizovati" => &[
        VerbDictionaryEntry { lemma: "legalizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "legti" => &[
        VerbDictionaryEntry { lemma: "legti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "lepetati" => &[
        VerbDictionaryEntry { lemma: "lepetati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "letěti" => &[
        VerbDictionaryEntry { lemma: "letěti", addition: "(leti)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ležati" => &[
        VerbDictionaryEntry { lemma: "ležati", addition: "(leži)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "lgati" => &[
        VerbDictionaryEntry { lemma: "lgati", addition: "(lže)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "liberalizovati" => &[
        VerbDictionaryEntry { lemma: "liberalizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "liceměriti" => &[
        VerbDictionaryEntry { lemma: "liceměriti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "likvidovati" => &[
        VerbDictionaryEntry { lemma: "likvidovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "limitovati" => &[
        VerbDictionaryEntry { lemma: "limitovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "linjati" => &[
        VerbDictionaryEntry { lemma: "linjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "linčevati" => &[
        VerbDictionaryEntry { lemma: "linčevati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "liti" => &[
        VerbDictionaryEntry { lemma: "liti", addition: "(lije)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "lizati" => &[
        VerbDictionaryEntry { lemma: "lizati", addition: "(liže)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "liznųti" => &[
        VerbDictionaryEntry { lemma: "liznųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "lišati" => &[
        VerbDictionaryEntry { lemma: "lišati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "lišiti" => &[
        VerbDictionaryEntry { lemma: "lišiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ljstiti" => &[
        VerbDictionaryEntry { lemma: "ljstiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ljubiti" => &[
        VerbDictionaryEntry { lemma: "ljubiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ljuljati" => &[
        VerbDictionaryEntry { lemma: "ljuljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "lobovati" => &[
        VerbDictionaryEntry { lemma: "lobovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "lojiti" => &[
        VerbDictionaryEntry { lemma: "lojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "lokati" => &[
        VerbDictionaryEntry { lemma: "lokati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "loknųti" => &[
        VerbDictionaryEntry { lemma: "loknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "lomiti" => &[
        VerbDictionaryEntry { lemma: "lomiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "loskotati" => &[
        VerbDictionaryEntry { lemma: "loskotati", addition: "(loskoče)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "loviti" => &[
        VerbDictionaryEntry { lemma: "loviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "lupiti" => &[
        VerbDictionaryEntry { lemma: "lupiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "luskati" => &[
        VerbDictionaryEntry { lemma: "luskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "lučiti" => &[
        VerbDictionaryEntry { lemma: "lučiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "luščiti" => &[
        VerbDictionaryEntry { lemma: "luščiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "lysěti" => &[
        VerbDictionaryEntry { lemma: "lysěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "lėskati" => &[
        VerbDictionaryEntry { lemma: "lėskati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "lėsknųti" => &[
        VerbDictionaryEntry { lemma: "lėsknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "lęgati" => &[
        VerbDictionaryEntry { lemma: "lęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "lęgti" => &[
        VerbDictionaryEntry { lemma: "lęgti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "lěpiti" => &[
        VerbDictionaryEntry { lemma: "lěpiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "lětati" => &[
        VerbDictionaryEntry { lemma: "lětati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "lětovati" => &[
        VerbDictionaryEntry { lemma: "lětovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "lězti" => &[
        VerbDictionaryEntry { lemma: "lězti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "lěčiti" => &[
        VerbDictionaryEntry { lemma: "lěčiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mahati" => &[
        VerbDictionaryEntry { lemma: "mahati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mahnųti" => &[
        VerbDictionaryEntry { lemma: "mahnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "maljevati" => &[
        VerbDictionaryEntry { lemma: "maljevati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "maltretovati" => &[
        VerbDictionaryEntry { lemma: "maltretovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "malěti" => &[
        VerbDictionaryEntry { lemma: "malěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mamiti" => &[
        VerbDictionaryEntry { lemma: "mamiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "manevrovati" => &[
        VerbDictionaryEntry { lemma: "manevrovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "manipulovati" => &[
        VerbDictionaryEntry { lemma: "manipulovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "marginalizovati" => &[
        VerbDictionaryEntry { lemma: "marginalizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "marinovati" => &[
        VerbDictionaryEntry { lemma: "marinovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "marširovati" => &[
        VerbDictionaryEntry { lemma: "marširovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "masakrovati" => &[
        VerbDictionaryEntry { lemma: "masakrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "masirovati" => &[
        VerbDictionaryEntry { lemma: "masirovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "maskovati" => &[
        VerbDictionaryEntry { lemma: "maskovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "mastiti" => &[
        VerbDictionaryEntry { lemma: "mastiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "masturbovati" => &[
        VerbDictionaryEntry { lemma: "masturbovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mazati" => &[
        VerbDictionaryEntry { lemma: "mazati", addition: "(maže)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "meblovati" => &[
        VerbDictionaryEntry { lemma: "meblovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "meditovati" => &[
        VerbDictionaryEntry { lemma: "meditovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mekati" => &[
        VerbDictionaryEntry { lemma: "mekati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mesti" => &[
        VerbDictionaryEntry { lemma: "mesti", addition: "(mete)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "metati" => &[
        VerbDictionaryEntry { lemma: "metati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "metnųti" => &[
        VerbDictionaryEntry { lemma: "metnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "mglati" => &[
        VerbDictionaryEntry { lemma: "mglati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "migati" => &[
        VerbDictionaryEntry { lemma: "migati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mignųti" => &[
        VerbDictionaryEntry { lemma: "mignųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "migrovati" => &[
        VerbDictionaryEntry { lemma: "migrovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "milovati" => &[
        VerbDictionaryEntry { lemma: "milovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "minimalizovati" => &[
        VerbDictionaryEntry { lemma: "minimalizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "minovati" => &[
        VerbDictionaryEntry { lemma: "minovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "minųti" => &[
        VerbDictionaryEntry { lemma: "minųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "miriti" => &[
        VerbDictionaryEntry { lemma: "miriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mjaukati" => &[
        VerbDictionaryEntry { lemma: "mjaukati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mjauknųti" => &[
        VerbDictionaryEntry { lemma: "mjauknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "mljaskati" => &[
        VerbDictionaryEntry { lemma: "mljaskati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mlåděti" => &[
        VerbDictionaryEntry { lemma: "mlåděti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mlåtiti" => &[
        VerbDictionaryEntry { lemma: "mlåtiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mlěti" => &[
        VerbDictionaryEntry { lemma: "mlěti", addition: "(melje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "množiti" => &[
        VerbDictionaryEntry { lemma: "množiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mněti" => &[
        VerbDictionaryEntry { lemma: "mněti", addition: "(mni)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mněvati" => &[
        VerbDictionaryEntry { lemma: "mněvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mobilizovati" => &[
        VerbDictionaryEntry { lemma: "mobilizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "modelovati" => &[
        VerbDictionaryEntry { lemma: "modelovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "modernizovati" => &[
        VerbDictionaryEntry { lemma: "modernizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "moderovati" => &[
        VerbDictionaryEntry { lemma: "moderovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "modifikovati" => &[
        VerbDictionaryEntry { lemma: "modifikovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "modriti" => &[
        VerbDictionaryEntry { lemma: "modriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "modrěti" => &[
        VerbDictionaryEntry { lemma: "modrěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mogti" => &[
        VerbDictionaryEntry { lemma: "mogti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "moknųti" => &[
        VerbDictionaryEntry { lemma: "moknųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mokriti" => &[
        VerbDictionaryEntry { lemma: "mokriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mokrěti" => &[
        VerbDictionaryEntry { lemma: "mokrěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "molestovati" => &[
        VerbDictionaryEntry { lemma: "molestovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "montovati" => &[
        VerbDictionaryEntry { lemma: "montovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "moralizovati" => &[
        VerbDictionaryEntry { lemma: "moralizovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "morati" => &[
        VerbDictionaryEntry { lemma: "morati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "moriti" => &[
        VerbDictionaryEntry { lemma: "moriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "motati" => &[
        VerbDictionaryEntry { lemma: "motati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "motivovati" => &[
        VerbDictionaryEntry { lemma: "motivovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "moćněti" => &[
        VerbDictionaryEntry { lemma: "moćněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "močiti" => &[
        VerbDictionaryEntry { lemma: "močiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mrdati" => &[
        VerbDictionaryEntry { lemma: "mrdati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mrdnųti" => &[
        VerbDictionaryEntry { lemma: "mrdnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "mrmjati" => &[
        VerbDictionaryEntry { lemma: "mrmjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mrznųti" => &[
        VerbDictionaryEntry { lemma: "mrznųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mråziti" => &[
        VerbDictionaryEntry { lemma: "mråziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mråzosušati" => &[
        VerbDictionaryEntry { lemma: "mråzosušati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mråzosušiti" => &[
        VerbDictionaryEntry { lemma: "mråzosušiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "mrčati" => &[
        VerbDictionaryEntry { lemma: "mrčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mrščiti" => &[
        VerbDictionaryEntry { lemma: "mrščiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mstiti" => &[
        VerbDictionaryEntry { lemma: "mstiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mukati" => &[
        VerbDictionaryEntry { lemma: "mukati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "musěti" => &[
        VerbDictionaryEntry { lemma: "musěti", addition: "(musi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mučati" => &[
        VerbDictionaryEntry { lemma: "mučati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "myliti" => &[
        VerbDictionaryEntry { lemma: "myliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mysliti" => &[
        VerbDictionaryEntry { lemma: "mysliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "myti" => &[
        VerbDictionaryEntry { lemma: "myti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "myškovati" => &[
        VerbDictionaryEntry { lemma: "myškovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mėdliti" => &[
        VerbDictionaryEntry { lemma: "mėdliti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mėčtati" => &[
        VerbDictionaryEntry { lemma: "mėčtati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mękčeti" => &[
        VerbDictionaryEntry { lemma: "mękčeti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mękčiti" => &[
        VerbDictionaryEntry { lemma: "mękčiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "męti" => &[
        VerbDictionaryEntry { lemma: "męti", addition: "(mne)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "męčkati" => &[
        VerbDictionaryEntry { lemma: "męčkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "měnjati" => &[
        VerbDictionaryEntry { lemma: "měnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "měriti" => &[
        VerbDictionaryEntry { lemma: "měriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "měriti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "měsiti" => &[
        VerbDictionaryEntry { lemma: "měsiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mětati" => &[
        VerbDictionaryEntry { lemma: "mětati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "měšati" => &[
        VerbDictionaryEntry { lemma: "měšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mŕknųti" => &[
        VerbDictionaryEntry { lemma: "mŕknųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mŕtvěti" => &[
        VerbDictionaryEntry { lemma: "mŕtvěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mųdrěti" => &[
        VerbDictionaryEntry { lemma: "mųdrěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mųtiti" => &[
        VerbDictionaryEntry { lemma: "mųtiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mųtněti" => &[
        VerbDictionaryEntry { lemma: "mųtněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mųčiti" => &[
        VerbDictionaryEntry { lemma: "mųčiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mȯlknųti" => &[
        VerbDictionaryEntry { lemma: "mȯlknųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "mȯlviti" => &[
        VerbDictionaryEntry { lemma: "mȯlviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "mȯlčati" => &[
        VerbDictionaryEntry { lemma: "mȯlčati", addition: "(mȯlči)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "nabajati" => &[
        VerbDictionaryEntry { lemma: "nabajati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "nabirati" => &[
        VerbDictionaryEntry { lemma: "nabirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nabiti" => &[
        VerbDictionaryEntry { lemma: "nabiti", addition: "(nabije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nabivati" => &[
        VerbDictionaryEntry { lemma: "nabivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nabrati" => &[
        VerbDictionaryEntry { lemma: "nabrati", addition: "(nabere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nabreknųti" => &[
        VerbDictionaryEntry { lemma: "nabreknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "nabuhati" => &[
        VerbDictionaryEntry { lemma: "nabuhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "nabuhnųti" => &[
        VerbDictionaryEntry { lemma: "nabuhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "nabyti" => &[
        VerbDictionaryEntry { lemma: "nabyti", addition: "(nabųde)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nabyvati" => &[
        VerbDictionaryEntry { lemma: "nabyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nabzděti" => &[
        VerbDictionaryEntry { lemma: "nabzděti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "nacionalizovati" => &[
        VerbDictionaryEntry { lemma: "nacionalizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nadati" => &[
        VerbDictionaryEntry { lemma: "nadati", addition: "(nada)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nadavati" => &[
        VerbDictionaryEntry { lemma: "nadavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nadględati" => &[
        VerbDictionaryEntry { lemma: "nadględati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nadględěti" => &[
        VerbDictionaryEntry { lemma: "nadględěti", addition: "(nadględi)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nadigrati" => &[
        VerbDictionaryEntry { lemma: "nadigrati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nadigryvati" => &[
        VerbDictionaryEntry { lemma: "nadigryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nadužiti" => &[
        VerbDictionaryEntry { lemma: "nadužiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "naduživati" => &[
        VerbDictionaryEntry { lemma: "naduživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nadyhati" => &[
        VerbDictionaryEntry { lemma: "nadyhati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nadzirati" => &[
        VerbDictionaryEntry { lemma: "nadzirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nadělati" => &[
        VerbDictionaryEntry { lemma: "nadělati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "naděti" => &[
        VerbDictionaryEntry { lemma: "naděti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "naděvati" => &[
        VerbDictionaryEntry { lemma: "naděvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nadųti" => &[
        VerbDictionaryEntry { lemma: "nadųti", addition: "(nadme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nadųvati" => &[
        VerbDictionaryEntry { lemma: "nadųvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nadȯhnųti" => &[
        VerbDictionaryEntry { lemma: "nadȯhnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "naganjati" => &[
        VerbDictionaryEntry { lemma: "naganjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nagnati" => &[
        VerbDictionaryEntry { lemma: "nagnati", addition: "(nagone)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nagnojiti" => &[
        VerbDictionaryEntry { lemma: "nagnojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nagnųti" => &[
        VerbDictionaryEntry { lemma: "nagnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nagovoriti" => &[
        VerbDictionaryEntry { lemma: "nagovoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nagrađati" => &[
        VerbDictionaryEntry { lemma: "nagrađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nagromaditi" => &[
        VerbDictionaryEntry { lemma: "nagromaditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nagromađati" => &[
        VerbDictionaryEntry { lemma: "nagromađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nagråditi" => &[
        VerbDictionaryEntry { lemma: "nagråditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nagrěti" => &[
        VerbDictionaryEntry { lemma: "nagrěti", addition: "(nagrěje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nagrěvati" => &[
        VerbDictionaryEntry { lemma: "nagrěvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nagybati" => &[
        VerbDictionaryEntry { lemma: "nagybati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nahmuriti" => &[
        VerbDictionaryEntry { lemma: "nahmuriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nahoditi" => &[
        VerbDictionaryEntry { lemma: "nahoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nahvatati" => &[
        VerbDictionaryEntry { lemma: "nahvatati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "naigrati" => &[
        VerbDictionaryEntry { lemma: "naigrati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "naigryvati" => &[
        VerbDictionaryEntry { lemma: "naigryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "najdti" => &[
        VerbDictionaryEntry { lemma: "najdti", addition: "(najde; našėl)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "naježiti" => &[
        VerbDictionaryEntry { lemma: "naježiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "najmati" => &[
        VerbDictionaryEntry { lemma: "najmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "najęti" => &[
        VerbDictionaryEntry { lemma: "najęti", addition: "(najme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nakalati" => &[
        VerbDictionaryEntry { lemma: "nakalati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nakapati" => &[
        VerbDictionaryEntry { lemma: "nakapati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "nakazati" => &[
        VerbDictionaryEntry { lemma: "nakazati", addition: "(nakaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nakazyvati" => &[
        VerbDictionaryEntry { lemma: "nakazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nakladati" => &[
        VerbDictionaryEntry { lemma: "nakladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "naklanjati" => &[
        VerbDictionaryEntry { lemma: "naklanjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nakloniti" => &[
        VerbDictionaryEntry { lemma: "nakloniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nakopati" => &[
        VerbDictionaryEntry { lemma: "nakopati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nakopyvati" => &[
        VerbDictionaryEntry { lemma: "nakopyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nakrmiti" => &[
        VerbDictionaryEntry { lemma: "nakrmiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nakryti" => &[
        VerbDictionaryEntry { lemma: "nakryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nakryvati" => &[
        VerbDictionaryEntry { lemma: "nakryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nakydati" => &[
        VerbDictionaryEntry { lemma: "nakydati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nakydyvati" => &[
        VerbDictionaryEntry { lemma: "nakydyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nalagati" => &[
        VerbDictionaryEntry { lemma: "nalagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nalegati" => &[
        VerbDictionaryEntry { lemma: "nalegati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "nalegti" => &[
        VerbDictionaryEntry { lemma: "nalegti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "naležati" => &[
        VerbDictionaryEntry { lemma: "naležati", addition: "(naleži)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "naliti" => &[
        VerbDictionaryEntry { lemma: "naliti", addition: "(nalije)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nalivati" => &[
        VerbDictionaryEntry { lemma: "nalivati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "naložiti" => &[
        VerbDictionaryEntry { lemma: "naložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "namastiti" => &[
        VerbDictionaryEntry { lemma: "namastiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "namazati" => &[
        VerbDictionaryEntry { lemma: "namazati", addition: "(namaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "namazyvati" => &[
        VerbDictionaryEntry { lemma: "namazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "namašćati" => &[
        VerbDictionaryEntry { lemma: "namašćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "namoknųti" => &[
        VerbDictionaryEntry { lemma: "namoknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "namokriti" => &[
        VerbDictionaryEntry { lemma: "namokriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "namontovati" => &[
        VerbDictionaryEntry { lemma: "namontovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "namotati" => &[
        VerbDictionaryEntry { lemma: "namotati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "namotyvati" => &[
        VerbDictionaryEntry { lemma: "namotyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "namočiti" => &[
        VerbDictionaryEntry { lemma: "namočiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "namyliti" => &[
        VerbDictionaryEntry { lemma: "namyliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "namyljati" => &[
        VerbDictionaryEntry { lemma: "namyljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "naměriti" => &[
        VerbDictionaryEntry { lemma: "naměriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "naměrjati" => &[
        VerbDictionaryEntry { lemma: "naměrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nanesti" => &[
        VerbDictionaryEntry { lemma: "nanesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nanizati" => &[
        VerbDictionaryEntry { lemma: "nanizati", addition: "(naniže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nanositi" => &[
        VerbDictionaryEntry { lemma: "nanositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "naostriti" => &[
        VerbDictionaryEntry { lemma: "naostriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "napadati" => &[
        VerbDictionaryEntry { lemma: "napadati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "napajati" => &[
        VerbDictionaryEntry { lemma: "napajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "naparfumovati" => &[
        VerbDictionaryEntry { lemma: "naparfumovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "napasti" => &[
        VerbDictionaryEntry { lemma: "napasti", addition: "(napade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "napasti", addition: "(napase)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "napečatati" => &[
        VerbDictionaryEntry { lemma: "napečatati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "naphati" => &[
        VerbDictionaryEntry { lemma: "naphati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "napihati" => &[
        VerbDictionaryEntry { lemma: "napihati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "napinati" => &[
        VerbDictionaryEntry { lemma: "napinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "napirati" => &[
        VerbDictionaryEntry { lemma: "napirati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "napisati" => &[
        VerbDictionaryEntry { lemma: "napisati", addition: "(napiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "napljuvati" => &[
        VerbDictionaryEntry { lemma: "napljuvati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "napojiti" => &[
        VerbDictionaryEntry { lemma: "napojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "napominati" => &[
        VerbDictionaryEntry { lemma: "napominati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "napomněti" => &[
        VerbDictionaryEntry { lemma: "napomněti", addition: "(napomni)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "napraviti" => &[
        VerbDictionaryEntry { lemma: "napraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "napravjati" => &[
        VerbDictionaryEntry { lemma: "napravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "naprěti" => &[
        VerbDictionaryEntry { lemma: "naprěti", addition: "(napre)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "naprųžiti" => &[
        VerbDictionaryEntry { lemma: "naprųžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "napustiti" => &[
        VerbDictionaryEntry { lemma: "napustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "napušćati" => &[
        VerbDictionaryEntry { lemma: "napušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "napęti" => &[
        VerbDictionaryEntry { lemma: "napęti", addition: "(napne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "napŕti" => &[
        VerbDictionaryEntry { lemma: "napŕti", addition: "(napre)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "napȯlniti" => &[
        VerbDictionaryEntry { lemma: "napȯlniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "napȯlnjati" => &[
        VerbDictionaryEntry { lemma: "napȯlnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "narastati" => &[
        VerbDictionaryEntry { lemma: "narastati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "naroditi" => &[
        VerbDictionaryEntry { lemma: "naroditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "narušati" => &[
        VerbDictionaryEntry { lemma: "narušati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "narušiti" => &[
        VerbDictionaryEntry { lemma: "narušiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "narvati" => &[
        VerbDictionaryEntry { lemma: "narvati", addition: "(narve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "narysovati" => &[
        VerbDictionaryEntry { lemma: "narysovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "naryvati" => &[
        VerbDictionaryEntry { lemma: "naryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "naråsti" => &[
        VerbDictionaryEntry { lemma: "naråsti", addition: "(naråste)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "narěkati" => &[
        VerbDictionaryEntry { lemma: "narěkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "narěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "narěkti" => &[
        VerbDictionaryEntry { lemma: "narěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "narězati" => &[
        VerbDictionaryEntry { lemma: "narězati", addition: "(narěže)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "narųčati" => &[
        VerbDictionaryEntry { lemma: "narųčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "narųčiti" => &[
        VerbDictionaryEntry { lemma: "narųčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nasaditi" => &[
        VerbDictionaryEntry { lemma: "nasaditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nasađati" => &[
        VerbDictionaryEntry { lemma: "nasađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nascati" => &[
        VerbDictionaryEntry { lemma: "nascati", addition: "(nasci)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "naseliti" => &[
        VerbDictionaryEntry { lemma: "naseliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "naseljati" => &[
        VerbDictionaryEntry { lemma: "naseljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nasilovati" => &[
        VerbDictionaryEntry { lemma: "nasilovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "naslađati" => &[
        VerbDictionaryEntry { lemma: "naslađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "naslåditi" => &[
        VerbDictionaryEntry { lemma: "naslåditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "naslěditi" => &[
        VerbDictionaryEntry { lemma: "naslěditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "naslědovati" => &[
        VerbDictionaryEntry { lemma: "naslědovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nasmoliti" => &[
        VerbDictionaryEntry { lemma: "nasmoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nastati" => &[
        VerbDictionaryEntry { lemma: "nastati", addition: "(nastane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "nastavati" => &[
        VerbDictionaryEntry { lemma: "nastavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "nastaviti" => &[
        VerbDictionaryEntry { lemma: "nastaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nastavjati" => &[
        VerbDictionaryEntry { lemma: "nastavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nastignųti" => &[
        VerbDictionaryEntry { lemma: "nastignųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nastojati" => &[
        VerbDictionaryEntry { lemma: "nastojati", addition: "(nastoji)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "nastojivati" => &[
        VerbDictionaryEntry { lemma: "nastojivati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "nastrajati" => &[
        VerbDictionaryEntry { lemma: "nastrajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nastrašiti" => &[
        VerbDictionaryEntry { lemma: "nastrašiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nastrojiti" => &[
        VerbDictionaryEntry { lemma: "nastrojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nastųpati" => &[
        VerbDictionaryEntry { lemma: "nastųpati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "nastųpiti" => &[
        VerbDictionaryEntry { lemma: "nastųpiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "nasypati" => &[
        VerbDictionaryEntry { lemma: "nasypati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nasytiti" => &[
        VerbDictionaryEntry { lemma: "nasytiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nasyćati" => &[
        VerbDictionaryEntry { lemma: "nasyćati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nasěkati" => &[
        VerbDictionaryEntry { lemma: "nasěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nasěkti" => &[
        VerbDictionaryEntry { lemma: "nasěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "natipkati" => &[
        VerbDictionaryEntry { lemma: "natipkati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "natirati" => &[
        VerbDictionaryEntry { lemma: "natirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "natiskati" => &[
        VerbDictionaryEntry { lemma: "natiskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "natisknųti" => &[
        VerbDictionaryEntry { lemma: "natisknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "natočiti" => &[
        VerbDictionaryEntry { lemma: "natočiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "natrěti" => &[
        VerbDictionaryEntry { lemma: "natrěti", addition: "(natre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "natvoriti" => &[
        VerbDictionaryEntry { lemma: "natvoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "natęgati" => &[
        VerbDictionaryEntry { lemma: "natęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "natęgnųti" => &[
        VerbDictionaryEntry { lemma: "natęgnųti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "natŕti" => &[
        VerbDictionaryEntry { lemma: "natŕti", addition: "(natre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "naučiti" => &[
        VerbDictionaryEntry { lemma: "naučiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "navesti" => &[
        VerbDictionaryEntry { lemma: "navesti", addition: "(navede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "naviti" => &[
        VerbDictionaryEntry { lemma: "naviti", addition: "(navije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "navivati" => &[
        VerbDictionaryEntry { lemma: "navivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "navlåžiti" => &[
        VerbDictionaryEntry { lemma: "navlåžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "navlěkati" => &[
        VerbDictionaryEntry { lemma: "navlěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "navlěkti" => &[
        VerbDictionaryEntry { lemma: "navlěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "navoditi" => &[
        VerbDictionaryEntry { lemma: "navoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "navodniti" => &[
        VerbDictionaryEntry { lemma: "navodniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "navodnjati" => &[
        VerbDictionaryEntry { lemma: "navodnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "navoščiti" => &[
        VerbDictionaryEntry { lemma: "navoščiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "navråžiti" => &[
        VerbDictionaryEntry { lemma: "navråžiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "navęzati" => &[
        VerbDictionaryEntry { lemma: "navęzati", addition: "(navęže)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "navęzati", addition: "(navęže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "navęzyvati" => &[
        VerbDictionaryEntry { lemma: "navęzyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "navęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "navěditi" => &[
        VerbDictionaryEntry { lemma: "navěditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "navěsiti" => &[
        VerbDictionaryEntry { lemma: "navěsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "navěđati" => &[
        VerbDictionaryEntry { lemma: "navěđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "navěšati" => &[
        VerbDictionaryEntry { lemma: "navěšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nazdravjati" => &[
        VerbDictionaryEntry { lemma: "nazdravjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "nazdråviti" => &[
        VerbDictionaryEntry { lemma: "nazdråviti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "naznačati" => &[
        VerbDictionaryEntry { lemma: "naznačati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "naznačiti" => &[
        VerbDictionaryEntry { lemma: "naznačiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nazvati" => &[
        VerbDictionaryEntry { lemma: "nazvati", addition: "(nazȯve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nazyvati" => &[
        VerbDictionaryEntry { lemma: "nazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "načaditi" => &[
        VerbDictionaryEntry { lemma: "načaditi", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "načinati" => &[
        VerbDictionaryEntry { lemma: "načinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "načrpati" => &[
        VerbDictionaryEntry { lemma: "načrpati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "načrtati" => &[
        VerbDictionaryEntry { lemma: "načrtati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "načęti" => &[
        VerbDictionaryEntry { lemma: "načęti", addition: "(načne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "našiti" => &[
        VerbDictionaryEntry { lemma: "našiti", addition: "(našije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "našivati" => &[
        VerbDictionaryEntry { lemma: "našivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "našėptati" => &[
        VerbDictionaryEntry { lemma: "našėptati", addition: "(našėpće)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nedoględnųti" => &[
        VerbDictionaryEntry { lemma: "nedoględnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nedoråzuměti" => &[
        VerbDictionaryEntry { lemma: "nedoråzuměti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nedoråzuměvati" => &[
        VerbDictionaryEntry { lemma: "nedoråzuměvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nedostavati" => &[
        VerbDictionaryEntry { lemma: "nedostavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "nehati" => &[
        VerbDictionaryEntry { lemma: "nehati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nenaviděti" => &[
        VerbDictionaryEntry { lemma: "nenaviděti", addition: "(nenavidi)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nepokojiti" => &[
        VerbDictionaryEntry { lemma: "nepokojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nesti" => &[
        VerbDictionaryEntry { lemma: "nesti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nevtralizovati" => &[
        VerbDictionaryEntry { lemma: "nevtralizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "nezadovaljati" => &[
        VerbDictionaryEntry { lemma: "nezadovaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nezadovoliti" => &[
        VerbDictionaryEntry { lemma: "nezadovoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "nizati" => &[
        VerbDictionaryEntry { lemma: "nizati", addition: "(niže)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "niščiti" => &[
        VerbDictionaryEntry { lemma: "niščiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "njuhati" => &[
        VerbDictionaryEntry { lemma: "njuhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "njuhnųti" => &[
        VerbDictionaryEntry { lemma: "njuhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "nominovati" => &[
        VerbDictionaryEntry { lemma: "nominovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nositi" => &[
        VerbDictionaryEntry { lemma: "nositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "noćevati" => &[
        VerbDictionaryEntry { lemma: "noćevati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "nuditi" => &[
        VerbDictionaryEntry { lemma: "nuditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "nyrjati" => &[
        VerbDictionaryEntry { lemma: "nyrjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "nyrnųti" => &[
        VerbDictionaryEntry { lemma: "nyrnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "něměti" => &[
        VerbDictionaryEntry { lemma: "něměti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "obagriti" => &[
        VerbDictionaryEntry { lemma: "obagriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obdariti" => &[
        VerbDictionaryEntry { lemma: "obdariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obdarjati" => &[
        VerbDictionaryEntry { lemma: "obdarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obdirati" => &[
        VerbDictionaryEntry { lemma: "obdirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obdivjati" => &[
        VerbDictionaryEntry { lemma: "obdivjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obdrěti" => &[
        VerbDictionaryEntry { lemma: "obdrěti", addition: "(obdre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obdumati" => &[
        VerbDictionaryEntry { lemma: "obdumati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obdŕti" => &[
        VerbDictionaryEntry { lemma: "obdŕti", addition: "(obdre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obezglåviti" => &[
        VerbDictionaryEntry { lemma: "obezglåviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obezglåvjati" => &[
        VerbDictionaryEntry { lemma: "obezglåvjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obezhrabrjati" => &[
        VerbDictionaryEntry { lemma: "obezhrabrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obezhråbriti" => &[
        VerbDictionaryEntry { lemma: "obezhråbriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obezpokojiti" => &[
        VerbDictionaryEntry { lemma: "obezpokojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obezsiliti" => &[
        VerbDictionaryEntry { lemma: "obezsiliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obezsiljati" => &[
        VerbDictionaryEntry { lemma: "obezsiljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obezuměti" => &[
        VerbDictionaryEntry { lemma: "obezuměti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "obezuměvati" => &[
        VerbDictionaryEntry { lemma: "obezuměvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "obezčėstiti" => &[
        VerbDictionaryEntry { lemma: "obezčėstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obglodati" => &[
        VerbDictionaryEntry { lemma: "obglodati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obględati" => &[
        VerbDictionaryEntry { lemma: "obględati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obględěti" => &[
        VerbDictionaryEntry { lemma: "obględěti", addition: "(obględi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obgovarjati" => &[
        VerbDictionaryEntry { lemma: "obgovarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obgovoriti" => &[
        VerbDictionaryEntry { lemma: "obgovoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obgryzati" => &[
        VerbDictionaryEntry { lemma: "obgryzati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obgryzti" => &[
        VerbDictionaryEntry { lemma: "obgryzti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obhoditi" => &[
        VerbDictionaryEntry { lemma: "obhoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obhvatiti" => &[
        VerbDictionaryEntry { lemma: "obhvatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obhvaćati" => &[
        VerbDictionaryEntry { lemma: "obhvaćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obiděti" => &[
        VerbDictionaryEntry { lemma: "obiděti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obimati" => &[
        VerbDictionaryEntry { lemma: "obimati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obiđati" => &[
        VerbDictionaryEntry { lemma: "obiđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "objasniti" => &[
        VerbDictionaryEntry { lemma: "objasniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "objasnjati" => &[
        VerbDictionaryEntry { lemma: "objasnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "objaviti" => &[
        VerbDictionaryEntry { lemma: "objaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "objavjati" => &[
        VerbDictionaryEntry { lemma: "objavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "objediniti" => &[
        VerbDictionaryEntry { lemma: "objediniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "objedinjati" => &[
        VerbDictionaryEntry { lemma: "objedinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "objehati" => &[
        VerbDictionaryEntry { lemma: "objehati", addition: "(objede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "objezditi" => &[
        VerbDictionaryEntry { lemma: "objezditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obježđati" => &[
        VerbDictionaryEntry { lemma: "obježđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "objęti" => &[
        VerbDictionaryEntry { lemma: "objęti", addition: "(obȯjme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obkaljati" => &[
        VerbDictionaryEntry { lemma: "obkaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obkoliti" => &[
        VerbDictionaryEntry { lemma: "obkoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obkradati" => &[
        VerbDictionaryEntry { lemma: "obkradati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obkrasti" => &[
        VerbDictionaryEntry { lemma: "obkrasti", addition: "(obkrade)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oblagati" => &[
        VerbDictionaryEntry { lemma: "oblagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oblajati" => &[
        VerbDictionaryEntry { lemma: "oblajati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oblajivati" => &[
        VerbDictionaryEntry { lemma: "oblajivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oblamyvati" => &[
        VerbDictionaryEntry { lemma: "oblamyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oblačati" => &[
        VerbDictionaryEntry { lemma: "oblačati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obleděněti" => &[
        VerbDictionaryEntry { lemma: "obleděněti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "obletěti" => &[
        VerbDictionaryEntry { lemma: "obletěti", addition: "(obleti)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oblinjati" => &[
        VerbDictionaryEntry { lemma: "oblinjati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obliti" => &[
        VerbDictionaryEntry { lemma: "obliti", addition: "(oblije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oblivati" => &[
        VerbDictionaryEntry { lemma: "oblivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oblizati" => &[
        VerbDictionaryEntry { lemma: "oblizati", addition: "(obliže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oblizyvati" => &[
        VerbDictionaryEntry { lemma: "oblizyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oblomiti" => &[
        VerbDictionaryEntry { lemma: "oblomiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obložiti" => &[
        VerbDictionaryEntry { lemma: "obložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oblupiti" => &[
        VerbDictionaryEntry { lemma: "oblupiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obluščiti" => &[
        VerbDictionaryEntry { lemma: "obluščiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oblysěti" => &[
        VerbDictionaryEntry { lemma: "oblysěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "oblåčiti" => &[
        VerbDictionaryEntry { lemma: "oblåčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oblėgčati" => &[
        VerbDictionaryEntry { lemma: "oblėgčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oblėgčiti" => &[
        VerbDictionaryEntry { lemma: "oblėgčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oblěkati" => &[
        VerbDictionaryEntry { lemma: "oblěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oblěkti" => &[
        VerbDictionaryEntry { lemma: "oblěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oblětati" => &[
        VerbDictionaryEntry { lemma: "oblětati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obmanyvati" => &[
        VerbDictionaryEntry { lemma: "obmanyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obmanųti" => &[
        VerbDictionaryEntry { lemma: "obmanųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obmazati" => &[
        VerbDictionaryEntry { lemma: "obmazati", addition: "(obmaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obmazyvati" => &[
        VerbDictionaryEntry { lemma: "obmazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obmeđati" => &[
        VerbDictionaryEntry { lemma: "obmeđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obmeđiti" => &[
        VerbDictionaryEntry { lemma: "obmeđiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obmotati" => &[
        VerbDictionaryEntry { lemma: "obmotati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obmotyvati" => &[
        VerbDictionaryEntry { lemma: "obmotyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obmysliti" => &[
        VerbDictionaryEntry { lemma: "obmysliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obmysljati" => &[
        VerbDictionaryEntry { lemma: "obmysljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obměniti" => &[
        VerbDictionaryEntry { lemma: "obměniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obměnjati" => &[
        VerbDictionaryEntry { lemma: "obměnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obnarodovati" => &[
        VerbDictionaryEntry { lemma: "obnarodovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "obnavjati" => &[
        VerbDictionaryEntry { lemma: "obnavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obnažati" => &[
        VerbDictionaryEntry { lemma: "obnažati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obnažiti" => &[
        VerbDictionaryEntry { lemma: "obnažiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obnesti" => &[
        VerbDictionaryEntry { lemma: "obnesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obniziti" => &[
        VerbDictionaryEntry { lemma: "obniziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obnižati" => &[
        VerbDictionaryEntry { lemma: "obnižati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obnjuhati" => &[
        VerbDictionaryEntry { lemma: "obnjuhati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obnjuhyvati" => &[
        VerbDictionaryEntry { lemma: "obnjuhyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obnositi" => &[
        VerbDictionaryEntry { lemma: "obnositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obnoviti" => &[
        VerbDictionaryEntry { lemma: "obnoviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obobćati" => &[
        VerbDictionaryEntry { lemma: "obobćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obobćiti" => &[
        VerbDictionaryEntry { lemma: "obobćiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obogatiti" => &[
        VerbDictionaryEntry { lemma: "obogatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obogatěti" => &[
        VerbDictionaryEntry { lemma: "obogatěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "obogaćati" => &[
        VerbDictionaryEntry { lemma: "obogaćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obožati" => &[
        VerbDictionaryEntry { lemma: "obožati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obradovati" => &[
        VerbDictionaryEntry { lemma: "obradovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obranjati" => &[
        VerbDictionaryEntry { lemma: "obranjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obraćati" => &[
        VerbDictionaryEntry { lemma: "obraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obriti" => &[
        VerbDictionaryEntry { lemma: "obriti", addition: "(obrije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obrvati" => &[
        VerbDictionaryEntry { lemma: "obrvati", addition: "(obrve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obrysovati" => &[
        VerbDictionaryEntry { lemma: "obrysovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obryvati" => &[
        VerbDictionaryEntry { lemma: "obryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obryzgati" => &[
        VerbDictionaryEntry { lemma: "obryzgati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obråbotati" => &[
        VerbDictionaryEntry { lemma: "obråbotati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obråbotyvati" => &[
        VerbDictionaryEntry { lemma: "obråbotyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obråniti" => &[
        VerbDictionaryEntry { lemma: "obråniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obråtiti" => &[
        VerbDictionaryEntry { lemma: "obråtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obrěmeniti" => &[
        VerbDictionaryEntry { lemma: "obrěmeniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obrěmenjati" => &[
        VerbDictionaryEntry { lemma: "obrěmenjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obrězati" => &[
        VerbDictionaryEntry { lemma: "obrězati", addition: "(obrěže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obsaditi" => &[
        VerbDictionaryEntry { lemma: "obsaditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obsađati" => &[
        VerbDictionaryEntry { lemma: "obsađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obscati" => &[
        VerbDictionaryEntry { lemma: "obscati", addition: "(obsci)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "observovati" => &[
        VerbDictionaryEntry { lemma: "observovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obslugovati" => &[
        VerbDictionaryEntry { lemma: "obslugovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obslužiti" => &[
        VerbDictionaryEntry { lemma: "obslužiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obslědovati" => &[
        VerbDictionaryEntry { lemma: "obslědovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obsrati" => &[
        VerbDictionaryEntry { lemma: "obsrati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obstajati" => &[
        VerbDictionaryEntry { lemma: "obstajati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "obstanavjati" => &[
        VerbDictionaryEntry { lemma: "obstanavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obstųpati" => &[
        VerbDictionaryEntry { lemma: "obstųpati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obstųpiti" => &[
        VerbDictionaryEntry { lemma: "obstųpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obsvětliti" => &[
        VerbDictionaryEntry { lemma: "obsvětliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obsvětljati" => &[
        VerbDictionaryEntry { lemma: "obsvětljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obsyhati" => &[
        VerbDictionaryEntry { lemma: "obsyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "obsypati" => &[
        VerbDictionaryEntry { lemma: "obsypati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obsypyvati" => &[
        VerbDictionaryEntry { lemma: "obsypyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obsęgati" => &[
        VerbDictionaryEntry { lemma: "obsęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obsęgnųti" => &[
        VerbDictionaryEntry { lemma: "obsęgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obsějati" => &[
        VerbDictionaryEntry { lemma: "obsějati", addition: "(obsěje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obsějivati" => &[
        VerbDictionaryEntry { lemma: "obsějivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obsěkati" => &[
        VerbDictionaryEntry { lemma: "obsěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obsěkti" => &[
        VerbDictionaryEntry { lemma: "obsěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obsȯhnųti" => &[
        VerbDictionaryEntry { lemma: "obsȯhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "obtestovati" => &[
        VerbDictionaryEntry { lemma: "obtestovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obtirati" => &[
        VerbDictionaryEntry { lemma: "obtirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obtrěti" => &[
        VerbDictionaryEntry { lemma: "obtrěti", addition: "(obtre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obtęžati" => &[
        VerbDictionaryEntry { lemma: "obtęžati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obtęžiti" => &[
        VerbDictionaryEntry { lemma: "obtęžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obtŕti" => &[
        VerbDictionaryEntry { lemma: "obtŕti", addition: "(obtre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obuzdati" => &[
        VerbDictionaryEntry { lemma: "obuzdati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obuzdyvati" => &[
        VerbDictionaryEntry { lemma: "obuzdyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obučati" => &[
        VerbDictionaryEntry { lemma: "obučati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obučiti" => &[
        VerbDictionaryEntry { lemma: "obučiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obvadnjati" => &[
        VerbDictionaryEntry { lemma: "obvadnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obvažati" => &[
        VerbDictionaryEntry { lemma: "obvažati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obvažiti" => &[
        VerbDictionaryEntry { lemma: "obvažiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obvesti" => &[
        VerbDictionaryEntry { lemma: "obvesti", addition: "(obvede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obviti" => &[
        VerbDictionaryEntry { lemma: "obviti", addition: "(obvije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obvivati" => &[
        VerbDictionaryEntry { lemma: "obvivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obvoditi" => &[
        VerbDictionaryEntry { lemma: "obvoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obvodniti" => &[
        VerbDictionaryEntry { lemma: "obvodniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obvęzati" => &[
        VerbDictionaryEntry { lemma: "obvęzati", addition: "(obvęže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obvęzyvati" => &[
        VerbDictionaryEntry { lemma: "obvęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obvěstiti" => &[
        VerbDictionaryEntry { lemma: "obvěstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obvěšćati" => &[
        VerbDictionaryEntry { lemma: "obvěšćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obyvati" => &[
        VerbDictionaryEntry { lemma: "obyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obćiti" => &[
        VerbDictionaryEntry { lemma: "obćiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "občudovati" => &[
        VerbDictionaryEntry { lemma: "občudovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obědati" => &[
        VerbDictionaryEntry { lemma: "obědati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true },
    ],
    "oběgati" => &[
        VerbDictionaryEntry { lemma: "oběgati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oběliti" => &[
        VerbDictionaryEntry { lemma: "oběliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obělěti" => &[
        VerbDictionaryEntry { lemma: "obělěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "oběćati" => &[
        VerbDictionaryEntry { lemma: "oběćati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oběćivati" => &[
        VerbDictionaryEntry { lemma: "oběćivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obŕnųti" => &[
        VerbDictionaryEntry { lemma: "obŕnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obžegti" => &[
        VerbDictionaryEntry { lemma: "obžegti", addition: "(obžže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "obžigati" => &[
        VerbDictionaryEntry { lemma: "obžigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "obȯjdti" => &[
        VerbDictionaryEntry { lemma: "obȯjdti", addition: "(obȯjde; obšėl)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ocěniti" => &[
        VerbDictionaryEntry { lemma: "ocěniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odbirati" => &[
        VerbDictionaryEntry { lemma: "odbirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odbiti" => &[
        VerbDictionaryEntry { lemma: "odbiti", addition: "(odbije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odbivati" => &[
        VerbDictionaryEntry { lemma: "odbivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odbrati" => &[
        VerbDictionaryEntry { lemma: "odbrati", addition: "(odbere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odběgati" => &[
        VerbDictionaryEntry { lemma: "odběgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "odběgti" => &[
        VerbDictionaryEntry { lemma: "odběgti", addition: "(odběži)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "oddaliti" => &[
        VerbDictionaryEntry { lemma: "oddaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oddaljati" => &[
        VerbDictionaryEntry { lemma: "oddaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oddati" => &[
        VerbDictionaryEntry { lemma: "oddati", addition: "(odda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oddavati" => &[
        VerbDictionaryEntry { lemma: "oddavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oddirati" => &[
        VerbDictionaryEntry { lemma: "oddirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oddrěti" => &[
        VerbDictionaryEntry { lemma: "oddrěti", addition: "(oddre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oddyhati" => &[
        VerbDictionaryEntry { lemma: "oddyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "odděliti" => &[
        VerbDictionaryEntry { lemma: "odděliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odděljati" => &[
        VerbDictionaryEntry { lemma: "odděljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oddŕti" => &[
        VerbDictionaryEntry { lemma: "oddŕti", addition: "(oddre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oddȯhnųti" => &[
        VerbDictionaryEntry { lemma: "oddȯhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "odgadati" => &[
        VerbDictionaryEntry { lemma: "odgadati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odgadyvati" => &[
        VerbDictionaryEntry { lemma: "odgadyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odgnųti" => &[
        VerbDictionaryEntry { lemma: "odgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odgovarjati" => &[
        VerbDictionaryEntry { lemma: "odgovarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odgovoriti" => &[
        VerbDictionaryEntry { lemma: "odgovoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odgrebati" => &[
        VerbDictionaryEntry { lemma: "odgrebati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odgrebti" => &[
        VerbDictionaryEntry { lemma: "odgrebti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odgryzati" => &[
        VerbDictionaryEntry { lemma: "odgryzati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odgryzti" => &[
        VerbDictionaryEntry { lemma: "odgryzti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odgybati" => &[
        VerbDictionaryEntry { lemma: "odgybati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odhoditi" => &[
        VerbDictionaryEntry { lemma: "odhoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "odidti" => &[
        VerbDictionaryEntry { lemma: "odidti", addition: "(odide; odšėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "odimati" => &[
        VerbDictionaryEntry { lemma: "odimati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odjehati" => &[
        VerbDictionaryEntry { lemma: "odjehati", addition: "(odjede)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "odježđati" => &[
        VerbDictionaryEntry { lemma: "odježđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "odjęti" => &[
        VerbDictionaryEntry { lemma: "odjęti", addition: "(odȯjme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odkazati" => &[
        VerbDictionaryEntry { lemma: "odkazati", addition: "(odkaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odkazyvati" => &[
        VerbDictionaryEntry { lemma: "odkazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odkladati" => &[
        VerbDictionaryEntry { lemma: "odkladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odklanjati" => &[
        VerbDictionaryEntry { lemma: "odklanjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odklejati" => &[
        VerbDictionaryEntry { lemma: "odklejati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odklejiti" => &[
        VerbDictionaryEntry { lemma: "odklejiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odključati" => &[
        VerbDictionaryEntry { lemma: "odključati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odključiti" => &[
        VerbDictionaryEntry { lemma: "odključiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odkloniti" => &[
        VerbDictionaryEntry { lemma: "odkloniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odkryti" => &[
        VerbDictionaryEntry { lemma: "odkryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odkryvati" => &[
        VerbDictionaryEntry { lemma: "odkryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odkupiti" => &[
        VerbDictionaryEntry { lemma: "odkupiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odkydati" => &[
        VerbDictionaryEntry { lemma: "odkydati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odkydnųti" => &[
        VerbDictionaryEntry { lemma: "odkydnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odkųsiti" => &[
        VerbDictionaryEntry { lemma: "odkųsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odkųšati" => &[
        VerbDictionaryEntry { lemma: "odkųšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odlamyvati" => &[
        VerbDictionaryEntry { lemma: "odlamyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odletěti" => &[
        VerbDictionaryEntry { lemma: "odletěti", addition: "(odleti)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "odliti" => &[
        VerbDictionaryEntry { lemma: "odliti", addition: "(odlije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odlivati" => &[
        VerbDictionaryEntry { lemma: "odlivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odličati" => &[
        VerbDictionaryEntry { lemma: "odličati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odličiti" => &[
        VerbDictionaryEntry { lemma: "odličiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odlomiti" => &[
        VerbDictionaryEntry { lemma: "odlomiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odložiti" => &[
        VerbDictionaryEntry { lemma: "odložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odlěpiti" => &[
        VerbDictionaryEntry { lemma: "odlěpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odlěpjati" => &[
        VerbDictionaryEntry { lemma: "odlěpjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odlětati" => &[
        VerbDictionaryEntry { lemma: "odlětati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "odmesti" => &[
        VerbDictionaryEntry { lemma: "odmesti", addition: "(odmete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odmetati" => &[
        VerbDictionaryEntry { lemma: "odmetati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odmetnųti" => &[
        VerbDictionaryEntry { lemma: "odmetnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odmstiti" => &[
        VerbDictionaryEntry { lemma: "odmstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odmětati" => &[
        VerbDictionaryEntry { lemma: "odmětati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odnesti" => &[
        VerbDictionaryEntry { lemma: "odnesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odnositi" => &[
        VerbDictionaryEntry { lemma: "odnositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odobriti" => &[
        VerbDictionaryEntry { lemma: "odobriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odobrjati" => &[
        VerbDictionaryEntry { lemma: "odobrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odolěti" => &[
        VerbDictionaryEntry { lemma: "odolěti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odolěvati" => &[
        VerbDictionaryEntry { lemma: "odolěvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odomašniti" => &[
        VerbDictionaryEntry { lemma: "odomašniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odomašnjati" => &[
        VerbDictionaryEntry { lemma: "odomašnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odosobniti" => &[
        VerbDictionaryEntry { lemma: "odosobniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odosobnjati" => &[
        VerbDictionaryEntry { lemma: "odosobnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odpadati" => &[
        VerbDictionaryEntry { lemma: "odpadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "odpasti" => &[
        VerbDictionaryEntry { lemma: "odpasti", addition: "(odpade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "odpečatati" => &[
        VerbDictionaryEntry { lemma: "odpečatati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odpečatyvati" => &[
        VerbDictionaryEntry { lemma: "odpečatyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odphati" => &[
        VerbDictionaryEntry { lemma: "odphati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odpihati" => &[
        VerbDictionaryEntry { lemma: "odpihati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odpiliti" => &[
        VerbDictionaryEntry { lemma: "odpiliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odpiljati" => &[
        VerbDictionaryEntry { lemma: "odpiljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odpinati" => &[
        VerbDictionaryEntry { lemma: "odpinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odpisati" => &[
        VerbDictionaryEntry { lemma: "odpisati", addition: "(odpiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odpisyvati" => &[
        VerbDictionaryEntry { lemma: "odpisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odplatiti" => &[
        VerbDictionaryEntry { lemma: "odplatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odplaćati" => &[
        VerbDictionaryEntry { lemma: "odplaćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odplesti" => &[
        VerbDictionaryEntry { lemma: "odplesti", addition: "(odplete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odpletati" => &[
        VerbDictionaryEntry { lemma: "odpletati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odpluti" => &[
        VerbDictionaryEntry { lemma: "odpluti", addition: "(odplove)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "odplyvati" => &[
        VerbDictionaryEntry { lemma: "odplyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "odplyvti" => &[
        VerbDictionaryEntry { lemma: "odplyvti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "odpovědati" => &[
        VerbDictionaryEntry { lemma: "odpovědati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "odpovědati", addition: "(+3)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odpověděti" => &[
        VerbDictionaryEntry { lemma: "odpověděti", addition: "(odpově)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odpočivati" => &[
        VerbDictionaryEntry { lemma: "odpočivati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "odpočęti" => &[
        VerbDictionaryEntry { lemma: "odpočęti", addition: "(odpočne)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "odpraviti" => &[
        VerbDictionaryEntry { lemma: "odpraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odpravjati" => &[
        VerbDictionaryEntry { lemma: "odpravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odprašati" => &[
        VerbDictionaryEntry { lemma: "odprašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odpråšiti" => &[
        VerbDictionaryEntry { lemma: "odpråšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odpustiti" => &[
        VerbDictionaryEntry { lemma: "odpustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odpušćati" => &[
        VerbDictionaryEntry { lemma: "odpušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odpęti" => &[
        VerbDictionaryEntry { lemma: "odpęti", addition: "(odpne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odpųditi" => &[
        VerbDictionaryEntry { lemma: "odpųditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odraditi" => &[
        VerbDictionaryEntry { lemma: "odraditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odraziti" => &[
        VerbDictionaryEntry { lemma: "odraziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odračati" => &[
        VerbDictionaryEntry { lemma: "odračati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odrađati" => &[
        VerbDictionaryEntry { lemma: "odrađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odražati" => &[
        VerbDictionaryEntry { lemma: "odražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odrestavrovati" => &[
        VerbDictionaryEntry { lemma: "odrestavrovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odročiti" => &[
        VerbDictionaryEntry { lemma: "odročiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odrvati" => &[
        VerbDictionaryEntry { lemma: "odrvati", addition: "(odrve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odryti" => &[
        VerbDictionaryEntry { lemma: "odryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odryvati" => &[
        VerbDictionaryEntry { lemma: "odryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odrěkati" => &[
        VerbDictionaryEntry { lemma: "odrěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odrěkti" => &[
        VerbDictionaryEntry { lemma: "odrěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odrězati" => &[
        VerbDictionaryEntry { lemma: "odrězati", addition: "(odrěže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odrězyvati" => &[
        VerbDictionaryEntry { lemma: "odrězyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odrųbati" => &[
        VerbDictionaryEntry { lemma: "odrųbati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odrųbyvati" => &[
        VerbDictionaryEntry { lemma: "odrųbyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odskakati" => &[
        VerbDictionaryEntry { lemma: "odskakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "odskočiti" => &[
        VerbDictionaryEntry { lemma: "odskočiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "odslanjati" => &[
        VerbDictionaryEntry { lemma: "odslanjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odslati" => &[
        VerbDictionaryEntry { lemma: "odslati", addition: "(odšlje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odsloniti" => &[
        VerbDictionaryEntry { lemma: "odsloniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odslužiti" => &[
        VerbDictionaryEntry { lemma: "odslužiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odstati" => &[
        VerbDictionaryEntry { lemma: "odstati", addition: "(odstane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "odstavati" => &[
        VerbDictionaryEntry { lemma: "odstavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "odstaviti" => &[
        VerbDictionaryEntry { lemma: "odstaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odstavjati" => &[
        VerbDictionaryEntry { lemma: "odstavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odstranjati" => &[
        VerbDictionaryEntry { lemma: "odstranjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odstrašati" => &[
        VerbDictionaryEntry { lemma: "odstrašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odstrašiti" => &[
        VerbDictionaryEntry { lemma: "odstrašiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odstrigati" => &[
        VerbDictionaryEntry { lemma: "odstrigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odstrigti" => &[
        VerbDictionaryEntry { lemma: "odstrigti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odstråniti" => &[
        VerbDictionaryEntry { lemma: "odstråniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odstrěliti" => &[
        VerbDictionaryEntry { lemma: "odstrěliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odstųpati" => &[
        VerbDictionaryEntry { lemma: "odstųpati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "odstųpati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odstųpiti" => &[
        VerbDictionaryEntry { lemma: "odstųpiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "odstųpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odsunųti" => &[
        VerbDictionaryEntry { lemma: "odsunųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odsuvati" => &[
        VerbDictionaryEntry { lemma: "odsuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odsyhati" => &[
        VerbDictionaryEntry { lemma: "odsyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "odsylati" => &[
        VerbDictionaryEntry { lemma: "odsylati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odsěděti" => &[
        VerbDictionaryEntry { lemma: "odsěděti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odsěkati" => &[
        VerbDictionaryEntry { lemma: "odsěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odsěkti" => &[
        VerbDictionaryEntry { lemma: "odsěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odsěđati" => &[
        VerbDictionaryEntry { lemma: "odsěđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odsųtstvovati" => &[
        VerbDictionaryEntry { lemma: "odsųtstvovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "odsȯhnųti" => &[
        VerbDictionaryEntry { lemma: "odsȯhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "odtajati" => &[
        VerbDictionaryEntry { lemma: "odtajati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odtekti" => &[
        VerbDictionaryEntry { lemma: "odtekti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "odtirati" => &[
        VerbDictionaryEntry { lemma: "odtirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odtiskati" => &[
        VerbDictionaryEntry { lemma: "odtiskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odtisknųti" => &[
        VerbDictionaryEntry { lemma: "odtisknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odtrěti" => &[
        VerbDictionaryEntry { lemma: "odtrěti", addition: "(odtre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odtęgati" => &[
        VerbDictionaryEntry { lemma: "odtęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odtęgnųti" => &[
        VerbDictionaryEntry { lemma: "odtęgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odtěkati" => &[
        VerbDictionaryEntry { lemma: "odtěkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "odtŕti" => &[
        VerbDictionaryEntry { lemma: "odtŕti", addition: "(odtre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odurěti" => &[
        VerbDictionaryEntry { lemma: "odurěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "odučati" => &[
        VerbDictionaryEntry { lemma: "odučati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odučiti" => &[
        VerbDictionaryEntry { lemma: "odučiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odvadnjati" => &[
        VerbDictionaryEntry { lemma: "odvadnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odvesti" => &[
        VerbDictionaryEntry { lemma: "odvesti", addition: "(odvede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odvezti" => &[
        VerbDictionaryEntry { lemma: "odvezti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odvinųti" => &[
        VerbDictionaryEntry { lemma: "odvinųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odvivati" => &[
        VerbDictionaryEntry { lemma: "odvivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odvlåčivati" => &[
        VerbDictionaryEntry { lemma: "odvlåčivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odvlěkati" => &[
        VerbDictionaryEntry { lemma: "odvlěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odvlěkti" => &[
        VerbDictionaryEntry { lemma: "odvlěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odvoditi" => &[
        VerbDictionaryEntry { lemma: "odvoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odvodniti" => &[
        VerbDictionaryEntry { lemma: "odvodniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odvoziti" => &[
        VerbDictionaryEntry { lemma: "odvoziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odvraćati" => &[
        VerbDictionaryEntry { lemma: "odvraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odvråtiti" => &[
        VerbDictionaryEntry { lemma: "odvråtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odvykati" => &[
        VerbDictionaryEntry { lemma: "odvykati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odvyknųti" => &[
        VerbDictionaryEntry { lemma: "odvyknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odvęzati" => &[
        VerbDictionaryEntry { lemma: "odvęzati", addition: "(odvęže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odvęzyvati" => &[
        VerbDictionaryEntry { lemma: "odvęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odvŕgati" => &[
        VerbDictionaryEntry { lemma: "odvŕgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odvŕgnųti" => &[
        VerbDictionaryEntry { lemma: "odvŕgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odzavisiti" => &[
        VerbDictionaryEntry { lemma: "odzavisiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odzvati" => &[
        VerbDictionaryEntry { lemma: "odzvati", addition: "(odzȯve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odzyvati" => &[
        VerbDictionaryEntry { lemma: "odzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odćuđati" => &[
        VerbDictionaryEntry { lemma: "odćuđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odćuđiti" => &[
        VerbDictionaryEntry { lemma: "odćuđiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odčajati" => &[
        VerbDictionaryEntry { lemma: "odčajati", addition: "(odčaje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odčajivati" => &[
        VerbDictionaryEntry { lemma: "odčajivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odčiniti" => &[
        VerbDictionaryEntry { lemma: "odčiniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odčinjati" => &[
        VerbDictionaryEntry { lemma: "odčinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odčitati" => &[
        VerbDictionaryEntry { lemma: "odčitati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odčityvati" => &[
        VerbDictionaryEntry { lemma: "odčityvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odčuvati" => &[
        VerbDictionaryEntry { lemma: "odčuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oděti" => &[
        VerbDictionaryEntry { lemma: "oděti", addition: "(oděne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oděvati" => &[
        VerbDictionaryEntry { lemma: "oděvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odšlupati" => &[
        VerbDictionaryEntry { lemma: "odšlupati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odšlupyvati" => &[
        VerbDictionaryEntry { lemma: "odšlupyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "odščepiti" => &[
        VerbDictionaryEntry { lemma: "odščepiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "odščepjati" => &[
        VerbDictionaryEntry { lemma: "odščepjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ogladiti" => &[
        VerbDictionaryEntry { lemma: "ogladiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oglašati" => &[
        VerbDictionaryEntry { lemma: "oglašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ogluhnųti" => &[
        VerbDictionaryEntry { lemma: "ogluhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "oglupěti" => &[
        VerbDictionaryEntry { lemma: "oglupěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "oglušeti" => &[
        VerbDictionaryEntry { lemma: "oglušeti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "oglušiti" => &[
        VerbDictionaryEntry { lemma: "oglušiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oglåsiti" => &[
        VerbDictionaryEntry { lemma: "oglåsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oględati" => &[
        VerbDictionaryEntry { lemma: "oględati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oględěti" => &[
        VerbDictionaryEntry { lemma: "oględěti", addition: "(oględi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ognojiti" => &[
        VerbDictionaryEntry { lemma: "ognojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ogoliti" => &[
        VerbDictionaryEntry { lemma: "ogoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ogorčati" => &[
        VerbDictionaryEntry { lemma: "ogorčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ogorčiti" => &[
        VerbDictionaryEntry { lemma: "ogorčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ogovarjati" => &[
        VerbDictionaryEntry { lemma: "ogovarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ogovoriti" => &[
        VerbDictionaryEntry { lemma: "ogovoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ograbiti" => &[
        VerbDictionaryEntry { lemma: "ograbiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ograbjati" => &[
        VerbDictionaryEntry { lemma: "ograbjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ograničati" => &[
        VerbDictionaryEntry { lemma: "ograničati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ograničiti" => &[
        VerbDictionaryEntry { lemma: "ograničiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ograđati" => &[
        VerbDictionaryEntry { lemma: "ograđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ogražati" => &[
        VerbDictionaryEntry { lemma: "ogražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ogroziti" => &[
        VerbDictionaryEntry { lemma: "ogroziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ogråditi" => &[
        VerbDictionaryEntry { lemma: "ogråditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ogrěti" => &[
        VerbDictionaryEntry { lemma: "ogrěti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ogrěvati" => &[
        VerbDictionaryEntry { lemma: "ogrěvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ohati" => &[
        VerbDictionaryEntry { lemma: "ohati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ohlađati" => &[
        VerbDictionaryEntry { lemma: "ohlađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ohlåditi" => &[
        VerbDictionaryEntry { lemma: "ohlåditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ohlåděti" => &[
        VerbDictionaryEntry { lemma: "ohlåděti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "ohranjati" => &[
        VerbDictionaryEntry { lemma: "ohranjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ohroměti" => &[
        VerbDictionaryEntry { lemma: "ohroměti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "ohråniti" => &[
        VerbDictionaryEntry { lemma: "ohråniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "okameniti" => &[
        VerbDictionaryEntry { lemma: "okameniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "okamenjati" => &[
        VerbDictionaryEntry { lemma: "okamenjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "okameněti" => &[
        VerbDictionaryEntry { lemma: "okameněti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "okazati" => &[
        VerbDictionaryEntry { lemma: "okazati", addition: "(okaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "okazyvati" => &[
        VerbDictionaryEntry { lemma: "okazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oklevetati" => &[
        VerbDictionaryEntry { lemma: "oklevetati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "okolorovati" => &[
        VerbDictionaryEntry { lemma: "okolorovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "okovati" => &[
        VerbDictionaryEntry { lemma: "okovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "okropiti" => &[
        VerbDictionaryEntry { lemma: "okropiti", addition: "(+5)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "okropjati" => &[
        VerbDictionaryEntry { lemma: "okropjati", addition: "(+5)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "okrėstiti" => &[
        VerbDictionaryEntry { lemma: "okrėstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "okrěpnųti" => &[
        VerbDictionaryEntry { lemma: "okrěpnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "okrųgliti" => &[
        VerbDictionaryEntry { lemma: "okrųgliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "okrųgljati" => &[
        VerbDictionaryEntry { lemma: "okrųgljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "okrųžati" => &[
        VerbDictionaryEntry { lemma: "okrųžati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "okrųžiti" => &[
        VerbDictionaryEntry { lemma: "okrųžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "okupovati" => &[
        VerbDictionaryEntry { lemma: "okupovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "omamiti" => &[
        VerbDictionaryEntry { lemma: "omamiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "omastiti" => &[
        VerbDictionaryEntry { lemma: "omastiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "omesti" => &[
        VerbDictionaryEntry { lemma: "omesti", addition: "(omete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "omlađati" => &[
        VerbDictionaryEntry { lemma: "omlađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "omlåditi" => &[
        VerbDictionaryEntry { lemma: "omlåditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "omlåděti" => &[
        VerbDictionaryEntry { lemma: "omlåděti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "omlěti" => &[
        VerbDictionaryEntry { lemma: "omlěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "omlěvati" => &[
        VerbDictionaryEntry { lemma: "omlěvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "omyti" => &[
        VerbDictionaryEntry { lemma: "omyti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "omětati" => &[
        VerbDictionaryEntry { lemma: "omětati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "omŕtvěti" => &[
        VerbDictionaryEntry { lemma: "omŕtvěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "omŕziti" => &[
        VerbDictionaryEntry { lemma: "omŕziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "omŕžati" => &[
        VerbDictionaryEntry { lemma: "omŕžati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "omųžiti" => &[
        VerbDictionaryEntry { lemma: "omųžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "onesměliti" => &[
        VerbDictionaryEntry { lemma: "onesměliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "onesměljati" => &[
        VerbDictionaryEntry { lemma: "onesměljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oněměti" => &[
        VerbDictionaryEntry { lemma: "oněměti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "opadati" => &[
        VerbDictionaryEntry { lemma: "opadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "opakovati" => &[
        VerbDictionaryEntry { lemma: "opakovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "opakovyvati" => &[
        VerbDictionaryEntry { lemma: "opakovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "opaliti" => &[
        VerbDictionaryEntry { lemma: "opaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "opaljati" => &[
        VerbDictionaryEntry { lemma: "opaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "opariti" => &[
        VerbDictionaryEntry { lemma: "opariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oparjati" => &[
        VerbDictionaryEntry { lemma: "oparjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "opasti" => &[
        VerbDictionaryEntry { lemma: "opasti", addition: "(opade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "opekati" => &[
        VerbDictionaryEntry { lemma: "opekati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "opisati" => &[
        VerbDictionaryEntry { lemma: "opisati", addition: "(opiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "opisyvati" => &[
        VerbDictionaryEntry { lemma: "opisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "opiti" => &[
        VerbDictionaryEntry { lemma: "opiti", addition: "(opije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "opivati" => &[
        VerbDictionaryEntry { lemma: "opivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oplakati" => &[
        VerbDictionaryEntry { lemma: "oplakati", addition: "(oplače)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oplakyvati" => &[
        VerbDictionaryEntry { lemma: "oplakyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "opljunųti" => &[
        VerbDictionaryEntry { lemma: "opljunųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "opljuvati" => &[
        VerbDictionaryEntry { lemma: "opljuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oploditi" => &[
        VerbDictionaryEntry { lemma: "oploditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oplođati" => &[
        VerbDictionaryEntry { lemma: "oplođati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "opluti" => &[
        VerbDictionaryEntry { lemma: "opluti", addition: "(oplove)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oplyvati" => &[
        VerbDictionaryEntry { lemma: "oplyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oplěti" => &[
        VerbDictionaryEntry { lemma: "oplěti", addition: "(oplěje/oplěve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oporędčati" => &[
        VerbDictionaryEntry { lemma: "oporędčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oporędčiti" => &[
        VerbDictionaryEntry { lemma: "oporędčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "opozdniti" => &[
        VerbDictionaryEntry { lemma: "opozdniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "opozdnjati" => &[
        VerbDictionaryEntry { lemma: "opozdnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "opoznati" => &[
        VerbDictionaryEntry { lemma: "opoznati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "opoznavati" => &[
        VerbDictionaryEntry { lemma: "opoznavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "opravdati" => &[
        VerbDictionaryEntry { lemma: "opravdati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "opravdyvati" => &[
        VerbDictionaryEntry { lemma: "opravdyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "opraviti" => &[
        VerbDictionaryEntry { lemma: "opraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "opravjati" => &[
        VerbDictionaryEntry { lemma: "opravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "opravniti" => &[
        VerbDictionaryEntry { lemma: "opravniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "opravnjati" => &[
        VerbDictionaryEntry { lemma: "opravnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oprovŕgati" => &[
        VerbDictionaryEntry { lemma: "oprovŕgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oprovŕgnųti" => &[
        VerbDictionaryEntry { lemma: "oprovŕgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "opråzdniti" => &[
        VerbDictionaryEntry { lemma: "opråzdniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "opråzdnjati" => &[
        VerbDictionaryEntry { lemma: "opråzdnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oprěděliti" => &[
        VerbDictionaryEntry { lemma: "oprěděliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oprěděljati" => &[
        VerbDictionaryEntry { lemma: "oprěděljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "optimizovati" => &[
        VerbDictionaryEntry { lemma: "optimizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "opublikovati" => &[
        VerbDictionaryEntry { lemma: "opublikovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "opuhati" => &[
        VerbDictionaryEntry { lemma: "opuhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "opuhnųti" => &[
        VerbDictionaryEntry { lemma: "opuhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "opustašati" => &[
        VerbDictionaryEntry { lemma: "opustašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "opustiti" => &[
        VerbDictionaryEntry { lemma: "opustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "opustošiti" => &[
        VerbDictionaryEntry { lemma: "opustošiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "opustěti" => &[
        VerbDictionaryEntry { lemma: "opustěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "opušćati" => &[
        VerbDictionaryEntry { lemma: "opušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "opyliti" => &[
        VerbDictionaryEntry { lemma: "opyliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "opyljati" => &[
        VerbDictionaryEntry { lemma: "opyljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "orati" => &[
        VerbDictionaryEntry { lemma: "orati", addition: "(oŕe)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "orašati" => &[
        VerbDictionaryEntry { lemma: "orašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "organizovati" => &[
        VerbDictionaryEntry { lemma: "organizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "oriti" => &[
        VerbDictionaryEntry { lemma: "oriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "orkestrovati" => &[
        VerbDictionaryEntry { lemma: "orkestrovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "orositi" => &[
        VerbDictionaryEntry { lemma: "orositi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "orųdovati" => &[
        VerbDictionaryEntry { lemma: "orųdovati", addition: "(+5)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "orųžiti" => &[
        VerbDictionaryEntry { lemma: "orųžiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "osaditi" => &[
        VerbDictionaryEntry { lemma: "osaditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osamotiti" => &[
        VerbDictionaryEntry { lemma: "osamotiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osađati" => &[
        VerbDictionaryEntry { lemma: "osađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "osedlati" => &[
        VerbDictionaryEntry { lemma: "osedlati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osiliti" => &[
        VerbDictionaryEntry { lemma: "osiliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osiljati" => &[
        VerbDictionaryEntry { lemma: "osiljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "osirotiti" => &[
        VerbDictionaryEntry { lemma: "osirotiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osirotěti" => &[
        VerbDictionaryEntry { lemma: "osirotěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "osivěti" => &[
        VerbDictionaryEntry { lemma: "osivěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "oskopiti" => &[
        VerbDictionaryEntry { lemma: "oskopiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oskubati" => &[
        VerbDictionaryEntry { lemma: "oskubati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oskubyvati" => &[
        VerbDictionaryEntry { lemma: "oskubyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oskvŕniti" => &[
        VerbDictionaryEntry { lemma: "oskvŕniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oskvŕnjati" => &[
        VerbDictionaryEntry { lemma: "oskvŕnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oslabiti" => &[
        VerbDictionaryEntry { lemma: "oslabiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oslabjati" => &[
        VerbDictionaryEntry { lemma: "oslabjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oslaběti" => &[
        VerbDictionaryEntry { lemma: "oslaběti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "osloženiti" => &[
        VerbDictionaryEntry { lemma: "osloženiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osloženjati" => &[
        VerbDictionaryEntry { lemma: "osloženjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oslåditi" => &[
        VerbDictionaryEntry { lemma: "oslåditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oslěpiti" => &[
        VerbDictionaryEntry { lemma: "oslěpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oslěpjati" => &[
        VerbDictionaryEntry { lemma: "oslěpjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oslěpnųti" => &[
        VerbDictionaryEntry { lemma: "oslěpnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "osmažiti" => &[
        VerbDictionaryEntry { lemma: "osmažiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osmoliti" => &[
        VerbDictionaryEntry { lemma: "osmoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osměliti" => &[
        VerbDictionaryEntry { lemma: "osměliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osměljati" => &[
        VerbDictionaryEntry { lemma: "osměljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "osnovati" => &[
        VerbDictionaryEntry { lemma: "osnovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osnovyvati" => &[
        VerbDictionaryEntry { lemma: "osnovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "osněžiti" => &[
        VerbDictionaryEntry { lemma: "osněžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osoliti" => &[
        VerbDictionaryEntry { lemma: "osoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osparjati" => &[
        VerbDictionaryEntry { lemma: "osparjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "osporiti" => &[
        VerbDictionaryEntry { lemma: "osporiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osråmiti" => &[
        VerbDictionaryEntry { lemma: "osråmiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ostarěti" => &[
        VerbDictionaryEntry { lemma: "ostarěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "ostati" => &[
        VerbDictionaryEntry { lemma: "ostati", addition: "(ostane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "ostavati" => &[
        VerbDictionaryEntry { lemma: "ostavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ostaviti" => &[
        VerbDictionaryEntry { lemma: "ostaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ostavjati" => &[
        VerbDictionaryEntry { lemma: "ostavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ostrigati" => &[
        VerbDictionaryEntry { lemma: "ostrigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ostrigti" => &[
        VerbDictionaryEntry { lemma: "ostrigti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ostriti" => &[
        VerbDictionaryEntry { lemma: "ostriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ostrugati" => &[
        VerbDictionaryEntry { lemma: "ostrugati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ostrěgati" => &[
        VerbDictionaryEntry { lemma: "ostrěgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ostrěgti" => &[
        VerbDictionaryEntry { lemma: "ostrěgti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ostuditi" => &[
        VerbDictionaryEntry { lemma: "ostuditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osušati" => &[
        VerbDictionaryEntry { lemma: "osušati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "osušiti" => &[
        VerbDictionaryEntry { lemma: "osušiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osvajati" => &[
        VerbDictionaryEntry { lemma: "osvajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "osvobađati" => &[
        VerbDictionaryEntry { lemma: "osvobađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "osvoboditi" => &[
        VerbDictionaryEntry { lemma: "osvoboditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osvojiti" => &[
        VerbDictionaryEntry { lemma: "osvojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "osvętiti" => &[
        VerbDictionaryEntry { lemma: "osvętiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osvęćati" => &[
        VerbDictionaryEntry { lemma: "osvęćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "osvětiti" => &[
        VerbDictionaryEntry { lemma: "osvětiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osvětliti" => &[
        VerbDictionaryEntry { lemma: "osvětliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osvětljati" => &[
        VerbDictionaryEntry { lemma: "osvětljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "osvěćati" => &[
        VerbDictionaryEntry { lemma: "osvěćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "osvěžati" => &[
        VerbDictionaryEntry { lemma: "osvěžati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "osvěžiti" => &[
        VerbDictionaryEntry { lemma: "osvěžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osědati" => &[
        VerbDictionaryEntry { lemma: "osědati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "osěsti" => &[
        VerbDictionaryEntry { lemma: "osěsti", addition: "(osěde)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "osųditi" => &[
        VerbDictionaryEntry { lemma: "osųditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osųđati" => &[
        VerbDictionaryEntry { lemma: "osųđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "osȯvrěmenniti" => &[
        VerbDictionaryEntry { lemma: "osȯvrěmenniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "osȯvrěmennjati" => &[
        VerbDictionaryEntry { lemma: "osȯvrěmennjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "otišati" => &[
        VerbDictionaryEntry { lemma: "otišati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "otišiti" => &[
        VerbDictionaryEntry { lemma: "otišiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "otraviti" => &[
        VerbDictionaryEntry { lemma: "otraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "otravjati" => &[
        VerbDictionaryEntry { lemma: "otravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "otrudniti" => &[
        VerbDictionaryEntry { lemma: "otrudniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "otrudnjati" => &[
        VerbDictionaryEntry { lemma: "otrudnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "otręsati" => &[
        VerbDictionaryEntry { lemma: "otręsati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "otręsti" => &[
        VerbDictionaryEntry { lemma: "otręsti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "otrězviti" => &[
        VerbDictionaryEntry { lemma: "otrězviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "otrězvjati" => &[
        VerbDictionaryEntry { lemma: "otrězvjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "otrězvěti" => &[
        VerbDictionaryEntry { lemma: "otrězvěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "otvarjati" => &[
        VerbDictionaryEntry { lemma: "otvarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "otvoriti" => &[
        VerbDictionaryEntry { lemma: "otvoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "otvŕděti" => &[
        VerbDictionaryEntry { lemma: "otvŕděti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "otęgčati" => &[
        VerbDictionaryEntry { lemma: "otęgčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "otęgčiti" => &[
        VerbDictionaryEntry { lemma: "otęgčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "otěniti" => &[
        VerbDictionaryEntry { lemma: "otěniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "otěnjati" => &[
        VerbDictionaryEntry { lemma: "otěnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "otųpiti" => &[
        VerbDictionaryEntry { lemma: "otųpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "otųpjati" => &[
        VerbDictionaryEntry { lemma: "otųpjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "otųpěti" => &[
        VerbDictionaryEntry { lemma: "otųpěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "ovdověti" => &[
        VerbDictionaryEntry { lemma: "ovdověti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "ovinųti" => &[
        VerbDictionaryEntry { lemma: "ovinųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ovivati" => &[
        VerbDictionaryEntry { lemma: "ovivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ovladnųti" => &[
        VerbDictionaryEntry { lemma: "ovladnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ovladyvati" => &[
        VerbDictionaryEntry { lemma: "ovladyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ovplyvniti" => &[
        VerbDictionaryEntry { lemma: "ovplyvniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ovplyvnjati" => &[
        VerbDictionaryEntry { lemma: "ovplyvnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ovějati" => &[
        VerbDictionaryEntry { lemma: "ovějati", addition: "(ověje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ovějivati" => &[
        VerbDictionaryEntry { lemma: "ovějivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ověnčati" => &[
        VerbDictionaryEntry { lemma: "ověnčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ověnčiti" => &[
        VerbDictionaryEntry { lemma: "ověnčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ověriti" => &[
        VerbDictionaryEntry { lemma: "ověriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ověrjati" => &[
        VerbDictionaryEntry { lemma: "ověrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ovȯlgnųti" => &[
        VerbDictionaryEntry { lemma: "ovȯlgnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "ozdabjati" => &[
        VerbDictionaryEntry { lemma: "ozdabjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ozdobiti" => &[
        VerbDictionaryEntry { lemma: "ozdobiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ozdravjati" => &[
        VerbDictionaryEntry { lemma: "ozdravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ozdråviti" => &[
        VerbDictionaryEntry { lemma: "ozdråviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ozdråvěti" => &[
        VerbDictionaryEntry { lemma: "ozdråvěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "ozeleniti" => &[
        VerbDictionaryEntry { lemma: "ozeleniti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "ozelenjati" => &[
        VerbDictionaryEntry { lemma: "ozelenjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ozlobiti" => &[
        VerbDictionaryEntry { lemma: "ozlobiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oznamenovati" => &[
        VerbDictionaryEntry { lemma: "oznamenovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oznamenovyvati" => &[
        VerbDictionaryEntry { lemma: "oznamenovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "oznanjati" => &[
        VerbDictionaryEntry { lemma: "oznanjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "označati" => &[
        VerbDictionaryEntry { lemma: "označati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "označiti" => &[
        VerbDictionaryEntry { lemma: "označiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ozvěrěti" => &[
        VerbDictionaryEntry { lemma: "ozvěrěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "ozębti" => &[
        VerbDictionaryEntry { lemma: "ozębti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "očariti" => &[
        VerbDictionaryEntry { lemma: "očariti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "očarovati" => &[
        VerbDictionaryEntry { lemma: "očarovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "očarovati", addition: "(+3)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "očarovyvati" => &[
        VerbDictionaryEntry { lemma: "očarovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "očarovyvati", addition: "(+3)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "očekyvati" => &[
        VerbDictionaryEntry { lemma: "očekyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "očistiti" => &[
        VerbDictionaryEntry { lemma: "očistiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "očišćati" => &[
        VerbDictionaryEntry { lemma: "očišćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "očrkati" => &[
        VerbDictionaryEntry { lemma: "očrkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "očrknųti" => &[
        VerbDictionaryEntry { lemma: "očrknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "očrniti" => &[
        VerbDictionaryEntry { lemma: "očrniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "očrnjati" => &[
        VerbDictionaryEntry { lemma: "očrnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ošalěti" => &[
        VerbDictionaryEntry { lemma: "ošalěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "oženiti" => &[
        VerbDictionaryEntry { lemma: "oženiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ožiti" => &[
        VerbDictionaryEntry { lemma: "ožiti", addition: "(ožive)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "oživati" => &[
        VerbDictionaryEntry { lemma: "oživati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "oživiti" => &[
        VerbDictionaryEntry { lemma: "oživiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "oživjati" => &[
        VerbDictionaryEntry { lemma: "oživjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ožȯltěti" => &[
        VerbDictionaryEntry { lemma: "ožȯltěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "padati" => &[
        VerbDictionaryEntry { lemma: "padati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pakovati" => &[
        VerbDictionaryEntry { lemma: "pakovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "palatalizovati" => &[
        VerbDictionaryEntry { lemma: "palatalizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "paliti" => &[
        VerbDictionaryEntry { lemma: "paliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pamętati" => &[
        VerbDictionaryEntry { lemma: "pamętati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "panikovati" => &[
        VerbDictionaryEntry { lemma: "panikovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "paralizovati" => &[
        VerbDictionaryEntry { lemma: "paralizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "parazitovati" => &[
        VerbDictionaryEntry { lemma: "parazitovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "parfumovati" => &[
        VerbDictionaryEntry { lemma: "parfumovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pariti" => &[
        VerbDictionaryEntry { lemma: "pariti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "parkovati" => &[
        VerbDictionaryEntry { lemma: "parkovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "pasti" => &[
        VerbDictionaryEntry { lemma: "pasti", addition: "(pade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "pasti", addition: "(pase)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "patronizovati" => &[
        VerbDictionaryEntry { lemma: "patronizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pauzovati" => &[
        VerbDictionaryEntry { lemma: "pauzovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pekti" => &[
        VerbDictionaryEntry { lemma: "pekti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pečatati" => &[
        VerbDictionaryEntry { lemma: "pečatati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "phati" => &[
        VerbDictionaryEntry { lemma: "phati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "phnųti" => &[
        VerbDictionaryEntry { lemma: "phnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pikirovati" => &[
        VerbDictionaryEntry { lemma: "pikirovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "piliti" => &[
        VerbDictionaryEntry { lemma: "piliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pinati" => &[
        VerbDictionaryEntry { lemma: "pinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pirovati" => &[
        VerbDictionaryEntry { lemma: "pirovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pisati" => &[
        VerbDictionaryEntry { lemma: "pisati", addition: "(piše)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "piskati" => &[
        VerbDictionaryEntry { lemma: "piskati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "piti" => &[
        VerbDictionaryEntry { lemma: "piti", addition: "(pije)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pišati" => &[
        VerbDictionaryEntry { lemma: "pišati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "piščeti" => &[
        VerbDictionaryEntry { lemma: "piščeti", addition: "(pišče)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "plakati" => &[
        VerbDictionaryEntry { lemma: "plakati", addition: "(plače)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "planovati" => &[
        VerbDictionaryEntry { lemma: "planovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "platiti" => &[
        VerbDictionaryEntry { lemma: "platiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "plavati" => &[
        VerbDictionaryEntry { lemma: "plavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pleskati" => &[
        VerbDictionaryEntry { lemma: "pleskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "plesknųti" => &[
        VerbDictionaryEntry { lemma: "plesknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "plesti" => &[
        VerbDictionaryEntry { lemma: "plesti", addition: "(plete)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pljunųti" => &[
        VerbDictionaryEntry { lemma: "pljunųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pljuskati" => &[
        VerbDictionaryEntry { lemma: "pljuskati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pljusknųti" => &[
        VerbDictionaryEntry { lemma: "pljusknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pljuvati" => &[
        VerbDictionaryEntry { lemma: "pljuvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ploditi" => &[
        VerbDictionaryEntry { lemma: "ploditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pluti" => &[
        VerbDictionaryEntry { lemma: "pluti", addition: "(plove)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "plyvti" => &[
        VerbDictionaryEntry { lemma: "plyvti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "plęsati" => &[
        VerbDictionaryEntry { lemma: "plęsati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "plěniti" => &[
        VerbDictionaryEntry { lemma: "plěniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "plěti" => &[
        VerbDictionaryEntry { lemma: "plěti", addition: "(plěje/plěve)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pobiti" => &[
        VerbDictionaryEntry { lemma: "pobiti", addition: "(pobije)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "poblågodariti" => &[
        VerbDictionaryEntry { lemma: "poblågodariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poblågoželati" => &[
        VerbDictionaryEntry { lemma: "poblågoželati", addition: "(+3)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pobuditi" => &[
        VerbDictionaryEntry { lemma: "pobuditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pobuđati" => &[
        VerbDictionaryEntry { lemma: "pobuđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "poběditi" => &[
        VerbDictionaryEntry { lemma: "poběditi", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "poběgti" => &[
        VerbDictionaryEntry { lemma: "poběgti", addition: "(poběži)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "poběđati" => &[
        VerbDictionaryEntry { lemma: "poběđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pocělovati" => &[
        VerbDictionaryEntry { lemma: "pocělovati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "podariti" => &[
        VerbDictionaryEntry { lemma: "podariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podati" => &[
        VerbDictionaryEntry { lemma: "podati", addition: "(poda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podavati" => &[
        VerbDictionaryEntry { lemma: "podavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podbirati" => &[
        VerbDictionaryEntry { lemma: "podbirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podbrati" => &[
        VerbDictionaryEntry { lemma: "podbrati", addition: "(podbere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poddati" => &[
        VerbDictionaryEntry { lemma: "poddati", addition: "(podda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poddavati" => &[
        VerbDictionaryEntry { lemma: "poddavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "poddŕžati" => &[
        VerbDictionaryEntry { lemma: "poddŕžati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poddŕživati" => &[
        VerbDictionaryEntry { lemma: "poddŕživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podględati" => &[
        VerbDictionaryEntry { lemma: "podględati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "podględěti" => &[
        VerbDictionaryEntry { lemma: "podględěti", addition: "(podględi)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "podgovarjati" => &[
        VerbDictionaryEntry { lemma: "podgovarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podgovoriti" => &[
        VerbDictionaryEntry { lemma: "podgovoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podgrěti" => &[
        VerbDictionaryEntry { lemma: "podgrěti", addition: "(podgrěje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podgrěvati" => &[
        VerbDictionaryEntry { lemma: "podgrěvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podhoditi" => &[
        VerbDictionaryEntry { lemma: "podhoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "podimati" => &[
        VerbDictionaryEntry { lemma: "podimati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podjęti" => &[
        VerbDictionaryEntry { lemma: "podjęti", addition: "(podȯjme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podkladati" => &[
        VerbDictionaryEntry { lemma: "podkladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podključati" => &[
        VerbDictionaryEntry { lemma: "podključati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podključiti" => &[
        VerbDictionaryEntry { lemma: "podključiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podkopati" => &[
        VerbDictionaryEntry { lemma: "podkopati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podkopyvati" => &[
        VerbDictionaryEntry { lemma: "podkopyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podkovati" => &[
        VerbDictionaryEntry { lemma: "podkovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podkovyvati" => &[
        VerbDictionaryEntry { lemma: "podkovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podkrěpiti" => &[
        VerbDictionaryEntry { lemma: "podkrěpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podkrěpjati" => &[
        VerbDictionaryEntry { lemma: "podkrěpjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podkupiti" => &[
        VerbDictionaryEntry { lemma: "podkupiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podkupovati" => &[
        VerbDictionaryEntry { lemma: "podkupovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podlagati" => &[
        VerbDictionaryEntry { lemma: "podlagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podležati" => &[
        VerbDictionaryEntry { lemma: "podležati", addition: "(podleži) (+3)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "podliti" => &[
        VerbDictionaryEntry { lemma: "podliti", addition: "(podlije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podlivati" => &[
        VerbDictionaryEntry { lemma: "podlivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podložiti" => &[
        VerbDictionaryEntry { lemma: "podložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podměniti" => &[
        VerbDictionaryEntry { lemma: "podměniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podměnjati" => &[
        VerbDictionaryEntry { lemma: "podměnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podnurjati" => &[
        VerbDictionaryEntry { lemma: "podnurjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podpaliti" => &[
        VerbDictionaryEntry { lemma: "podpaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podpaljati" => &[
        VerbDictionaryEntry { lemma: "podpaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podpirati" => &[
        VerbDictionaryEntry { lemma: "podpirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podpisati" => &[
        VerbDictionaryEntry { lemma: "podpisati", addition: "(podpiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podpisyvati" => &[
        VerbDictionaryEntry { lemma: "podpisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podporiti" => &[
        VerbDictionaryEntry { lemma: "podporiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podråzděliti" => &[
        VerbDictionaryEntry { lemma: "podråzděliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podråzděljati" => &[
        VerbDictionaryEntry { lemma: "podråzděljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podręditi" => &[
        VerbDictionaryEntry { lemma: "podręditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podręđati" => &[
        VerbDictionaryEntry { lemma: "podręđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podslušati" => &[
        VerbDictionaryEntry { lemma: "podslušati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podslušivati" => &[
        VerbDictionaryEntry { lemma: "podslušivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podstrěkati" => &[
        VerbDictionaryEntry { lemma: "podstrěkati", addition: "(podstrěče)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podstrěknųti" => &[
        VerbDictionaryEntry { lemma: "podstrěknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podtiskati" => &[
        VerbDictionaryEntry { lemma: "podtiskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podtisknųti" => &[
        VerbDictionaryEntry { lemma: "podtisknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podtrimati" => &[
        VerbDictionaryEntry { lemma: "podtrimati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podtrimyvati" => &[
        VerbDictionaryEntry { lemma: "podtrimyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podvajati" => &[
        VerbDictionaryEntry { lemma: "podvajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podvažati" => &[
        VerbDictionaryEntry { lemma: "podvažati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podvažiti" => &[
        VerbDictionaryEntry { lemma: "podvažiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podvisiti" => &[
        VerbDictionaryEntry { lemma: "podvisiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podvojiti" => &[
        VerbDictionaryEntry { lemma: "podvojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podvŕgati" => &[
        VerbDictionaryEntry { lemma: "podvŕgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podvŕgnųti" => &[
        VerbDictionaryEntry { lemma: "podvŕgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podzirati" => &[
        VerbDictionaryEntry { lemma: "podzirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podzrěti" => &[
        VerbDictionaryEntry { lemma: "podzrěti", addition: "(podzri)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podzrěvati" => &[
        VerbDictionaryEntry { lemma: "podzrěvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podčrkati" => &[
        VerbDictionaryEntry { lemma: "podčrkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "podčrknųti" => &[
        VerbDictionaryEntry { lemma: "podčrknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podękovati" => &[
        VerbDictionaryEntry { lemma: "podękovati", addition: "(+3)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "poděliti" => &[
        VerbDictionaryEntry { lemma: "poděliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "podȯjdti" => &[
        VerbDictionaryEntry { lemma: "podȯjdti", addition: "(podȯjde; podšėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pogaršati" => &[
        VerbDictionaryEntry { lemma: "pogaršati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pogladiti" => &[
        VerbDictionaryEntry { lemma: "pogladiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poględati" => &[
        VerbDictionaryEntry { lemma: "poględati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "poględnųti" => &[
        VerbDictionaryEntry { lemma: "poględnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "poględyvati" => &[
        VerbDictionaryEntry { lemma: "poględyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "poglųbiti" => &[
        VerbDictionaryEntry { lemma: "poglųbiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poglųbjati" => &[
        VerbDictionaryEntry { lemma: "poglųbjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pogoršiti" => &[
        VerbDictionaryEntry { lemma: "pogoršiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pogovoriti" => &[
        VerbDictionaryEntry { lemma: "pogovoriti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pogrebati" => &[
        VerbDictionaryEntry { lemma: "pogrebati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pogrebti" => &[
        VerbDictionaryEntry { lemma: "pogrebti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pogrěšiti" => &[
        VerbDictionaryEntry { lemma: "pogrěšiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pogrųziti" => &[
        VerbDictionaryEntry { lemma: "pogrųziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pogrųžati" => &[
        VerbDictionaryEntry { lemma: "pogrųžati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pogynati" => &[
        VerbDictionaryEntry { lemma: "pogynati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pogynųti" => &[
        VerbDictionaryEntry { lemma: "pogynųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pogȯltnųti" => &[
        VerbDictionaryEntry { lemma: "pogȯltnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pohristijaniti" => &[
        VerbDictionaryEntry { lemma: "pohristijaniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pohristijanjati" => &[
        VerbDictionaryEntry { lemma: "pohristijanjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pohvaliti" => &[
        VerbDictionaryEntry { lemma: "pohvaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pohytiti" => &[
        VerbDictionaryEntry { lemma: "pohytiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pohyćati" => &[
        VerbDictionaryEntry { lemma: "pohyćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "poiskati" => &[
        VerbDictionaryEntry { lemma: "poiskati", addition: "(poišče)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poiskyvati" => &[
        VerbDictionaryEntry { lemma: "poiskyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pojdti" => &[
        VerbDictionaryEntry { lemma: "pojdti", addition: "(pojde; pošėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pojehati" => &[
        VerbDictionaryEntry { lemma: "pojehati", addition: "(pojede)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pojiti" => &[
        VerbDictionaryEntry { lemma: "pojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pojmati" => &[
        VerbDictionaryEntry { lemma: "pojmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pojęti" => &[
        VerbDictionaryEntry { lemma: "pojęti", addition: "(pojme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pokarati" => &[
        VerbDictionaryEntry { lemma: "pokarati", addition: "(pokare)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "pokarati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pokarjati" => &[
        VerbDictionaryEntry { lemma: "pokarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pokazati" => &[
        VerbDictionaryEntry { lemma: "pokazati", addition: "(pokaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pokazyvati" => &[
        VerbDictionaryEntry { lemma: "pokazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pokladati" => &[
        VerbDictionaryEntry { lemma: "pokladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "poklicati" => &[
        VerbDictionaryEntry { lemma: "poklicati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pokoriti" => &[
        VerbDictionaryEntry { lemma: "pokoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pokositi" => &[
        VerbDictionaryEntry { lemma: "pokositi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pokryti" => &[
        VerbDictionaryEntry { lemma: "pokryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pokryvati" => &[
        VerbDictionaryEntry { lemma: "pokryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pokrėstiti" => &[
        VerbDictionaryEntry { lemma: "pokrėstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pokusiti" => &[
        VerbDictionaryEntry { lemma: "pokusiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pokušati" => &[
        VerbDictionaryEntry { lemma: "pokušati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "polakovati" => &[
        VerbDictionaryEntry { lemma: "polakovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poletěti" => &[
        VerbDictionaryEntry { lemma: "poletěti", addition: "(poleti)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "polirati" => &[
        VerbDictionaryEntry { lemma: "polirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "politi" => &[
        VerbDictionaryEntry { lemma: "politi", addition: "(polije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "polivati" => &[
        VerbDictionaryEntry { lemma: "polivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "poljzovati" => &[
        VerbDictionaryEntry { lemma: "poljzovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "položiti" => &[
        VerbDictionaryEntry { lemma: "položiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "polučati" => &[
        VerbDictionaryEntry { lemma: "polučati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "polučiti" => &[
        VerbDictionaryEntry { lemma: "polučiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "polězti" => &[
        VerbDictionaryEntry { lemma: "polězti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pomagati" => &[
        VerbDictionaryEntry { lemma: "pomagati", addition: "(+3)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pomazati" => &[
        VerbDictionaryEntry { lemma: "pomazati", addition: "(pomaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pomilovati" => &[
        VerbDictionaryEntry { lemma: "pomilovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pomiriti" => &[
        VerbDictionaryEntry { lemma: "pomiriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pomněti" => &[
        VerbDictionaryEntry { lemma: "pomněti", addition: "(pomni)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pomogti" => &[
        VerbDictionaryEntry { lemma: "pomogti", addition: "(+3)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pomyliti" => &[
        VerbDictionaryEntry { lemma: "pomyliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pomysliti" => &[
        VerbDictionaryEntry { lemma: "pomysliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poměstiti" => &[
        VerbDictionaryEntry { lemma: "poměstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poměšćati" => &[
        VerbDictionaryEntry { lemma: "poměšćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "poniziti" => &[
        VerbDictionaryEntry { lemma: "poniziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ponižati" => &[
        VerbDictionaryEntry { lemma: "ponižati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "popiti" => &[
        VerbDictionaryEntry { lemma: "popiti", addition: "(popije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "popivati" => &[
        VerbDictionaryEntry { lemma: "popivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "poplaviti" => &[
        VerbDictionaryEntry { lemma: "poplaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poplavjati" => &[
        VerbDictionaryEntry { lemma: "poplavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "popluti" => &[
        VerbDictionaryEntry { lemma: "popluti", addition: "(poplove)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "poplyvti" => &[
        VerbDictionaryEntry { lemma: "poplyvti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "popraviti" => &[
        VerbDictionaryEntry { lemma: "popraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "popravjati" => &[
        VerbDictionaryEntry { lemma: "popravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "poprobovati" => &[
        VerbDictionaryEntry { lemma: "poprobovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poprositi" => &[
        VerbDictionaryEntry { lemma: "poprositi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "popularizovati" => &[
        VerbDictionaryEntry { lemma: "popularizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "popustiti" => &[
        VerbDictionaryEntry { lemma: "popustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "popušćati" => &[
        VerbDictionaryEntry { lemma: "popušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "popȯlzati" => &[
        VerbDictionaryEntry { lemma: "popȯlzati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "popȯlzti" => &[
        VerbDictionaryEntry { lemma: "popȯlzti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "poraditi" => &[
        VerbDictionaryEntry { lemma: "poraditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poraniti" => &[
        VerbDictionaryEntry { lemma: "poraniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poraziti" => &[
        VerbDictionaryEntry { lemma: "poraziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "porađati" => &[
        VerbDictionaryEntry { lemma: "porađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "poražati" => &[
        VerbDictionaryEntry { lemma: "poražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "poroditi" => &[
        VerbDictionaryEntry { lemma: "poroditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "porušiti" => &[
        VerbDictionaryEntry { lemma: "porušiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poråbiti" => &[
        VerbDictionaryEntry { lemma: "poråbiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poråbovati" => &[
        VerbDictionaryEntry { lemma: "poråbovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "poråsti" => &[
        VerbDictionaryEntry { lemma: "poråsti", addition: "(poråste)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "poråvniti" => &[
        VerbDictionaryEntry { lemma: "poråvniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "porųbati" => &[
        VerbDictionaryEntry { lemma: "porųbati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poslati" => &[
        VerbDictionaryEntry { lemma: "poslati", addition: "(pošlje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poslušati" => &[
        VerbDictionaryEntry { lemma: "poslušati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "posoliti" => &[
        VerbDictionaryEntry { lemma: "posoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pospati" => &[
        VerbDictionaryEntry { lemma: "pospati", addition: "(pospi)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pospěšiti" => &[
        VerbDictionaryEntry { lemma: "pospěšiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "posrěbriti" => &[
        VerbDictionaryEntry { lemma: "posrěbriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "posrěbrjati" => &[
        VerbDictionaryEntry { lemma: "posrěbrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "posrědkovati" => &[
        VerbDictionaryEntry { lemma: "posrědkovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "postanavjati" => &[
        VerbDictionaryEntry { lemma: "postanavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "postanoviti" => &[
        VerbDictionaryEntry { lemma: "postanoviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "postarěti" => &[
        VerbDictionaryEntry { lemma: "postarěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "postaviti" => &[
        VerbDictionaryEntry { lemma: "postaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "postavjati" => &[
        VerbDictionaryEntry { lemma: "postavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "posteliti" => &[
        VerbDictionaryEntry { lemma: "posteliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "postigati" => &[
        VerbDictionaryEntry { lemma: "postigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "postignųti" => &[
        VerbDictionaryEntry { lemma: "postignųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "postlati" => &[
        VerbDictionaryEntry { lemma: "postlati", addition: "(postelje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "postulovati" => &[
        VerbDictionaryEntry { lemma: "postulovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "postųpati" => &[
        VerbDictionaryEntry { lemma: "postųpati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "postųpiti" => &[
        VerbDictionaryEntry { lemma: "postųpiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "posvętiti" => &[
        VerbDictionaryEntry { lemma: "posvętiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "posvęćati" => &[
        VerbDictionaryEntry { lemma: "posvęćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "posvědčati" => &[
        VerbDictionaryEntry { lemma: "posvědčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "posvědčiti" => &[
        VerbDictionaryEntry { lemma: "posvědčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "posvědčiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "posylati" => &[
        VerbDictionaryEntry { lemma: "posylati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "posědati" => &[
        VerbDictionaryEntry { lemma: "posědati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "posějati" => &[
        VerbDictionaryEntry { lemma: "posějati", addition: "(posěje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "posěkati" => &[
        VerbDictionaryEntry { lemma: "posěkati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "posěkti" => &[
        VerbDictionaryEntry { lemma: "posěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "posěsti" => &[
        VerbDictionaryEntry { lemma: "posěsti", addition: "(posěde)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "posětiti" => &[
        VerbDictionaryEntry { lemma: "posětiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "posěćati" => &[
        VerbDictionaryEntry { lemma: "posěćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "posȯvětovati" => &[
        VerbDictionaryEntry { lemma: "posȯvětovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "potapjati" => &[
        VerbDictionaryEntry { lemma: "potapjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "potelefonovati" => &[
        VerbDictionaryEntry { lemma: "potelefonovati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "potopiti" => &[
        VerbDictionaryEntry { lemma: "potopiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "potrajati" => &[
        VerbDictionaryEntry { lemma: "potrajati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "potrvati" => &[
        VerbDictionaryEntry { lemma: "potrvati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "potręsati" => &[
        VerbDictionaryEntry { lemma: "potręsati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "potręsti" => &[
        VerbDictionaryEntry { lemma: "potręsti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "potrěbovati" => &[
        VerbDictionaryEntry { lemma: "potrěbovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "potvŕditi" => &[
        VerbDictionaryEntry { lemma: "potvŕditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "potvŕđati" => &[
        VerbDictionaryEntry { lemma: "potvŕđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "potėmněti" => &[
        VerbDictionaryEntry { lemma: "potėmněti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "potěti" => &[
        VerbDictionaryEntry { lemma: "potěti", addition: "(poti)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "poučati" => &[
        VerbDictionaryEntry { lemma: "poučati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "poučiti" => &[
        VerbDictionaryEntry { lemma: "poučiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "povaliti" => &[
        VerbDictionaryEntry { lemma: "povaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "považati" => &[
        VerbDictionaryEntry { lemma: "považati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "považiti" => &[
        VerbDictionaryEntry { lemma: "považiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "povraćati" => &[
        VerbDictionaryEntry { lemma: "povraćati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "povråtiti" => &[
        VerbDictionaryEntry { lemma: "povråtiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "povstati" => &[
        VerbDictionaryEntry { lemma: "povstati", addition: "(povstane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "povstavati" => &[
        VerbDictionaryEntry { lemma: "povstavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "povtarjati" => &[
        VerbDictionaryEntry { lemma: "povtarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "povtoriti" => &[
        VerbDictionaryEntry { lemma: "povtoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "povysiti" => &[
        VerbDictionaryEntry { lemma: "povysiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "povyšati" => &[
        VerbDictionaryEntry { lemma: "povyšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "povęzati" => &[
        VerbDictionaryEntry { lemma: "povęzati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "povęzyvati" => &[
        VerbDictionaryEntry { lemma: "povęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "povědati" => &[
        VerbDictionaryEntry { lemma: "povědati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pověděti" => &[
        VerbDictionaryEntry { lemma: "pověděti", addition: "(pově)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pověriti" => &[
        VerbDictionaryEntry { lemma: "pověriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pověrjati" => &[
        VerbDictionaryEntry { lemma: "pověrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pověsiti" => &[
        VerbDictionaryEntry { lemma: "pověsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "povŕnųti" => &[
        VerbDictionaryEntry { lemma: "povŕnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "povŕtati" => &[
        VerbDictionaryEntry { lemma: "povŕtati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pozajmati" => &[
        VerbDictionaryEntry { lemma: "pozajmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pozajęti" => &[
        VerbDictionaryEntry { lemma: "pozajęti", addition: "(pozajme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pozastaviti" => &[
        VerbDictionaryEntry { lemma: "pozastaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pozdravjati" => &[
        VerbDictionaryEntry { lemma: "pozdravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pozdråviti" => &[
        VerbDictionaryEntry { lemma: "pozdråviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pozlaćati" => &[
        VerbDictionaryEntry { lemma: "pozlaćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pozlåtiti" => &[
        VerbDictionaryEntry { lemma: "pozlåtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poznati" => &[
        VerbDictionaryEntry { lemma: "poznati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poznavati" => &[
        VerbDictionaryEntry { lemma: "poznavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pozovati" => &[
        VerbDictionaryEntry { lemma: "pozovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pozvaljati" => &[
        VerbDictionaryEntry { lemma: "pozvaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pozvati" => &[
        VerbDictionaryEntry { lemma: "pozvati", addition: "(pozȯve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pozvoliti" => &[
        VerbDictionaryEntry { lemma: "pozvoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pozvoniti" => &[
        VerbDictionaryEntry { lemma: "pozvoniti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "počekati" => &[
        VerbDictionaryEntry { lemma: "počekati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "počinati" => &[
        VerbDictionaryEntry { lemma: "počinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "počrveněti" => &[
        VerbDictionaryEntry { lemma: "počrveněti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "počęti" => &[
        VerbDictionaryEntry { lemma: "počęti", addition: "(počne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poškoditi" => &[
        VerbDictionaryEntry { lemma: "poškoditi", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "poščęditi" => &[
        VerbDictionaryEntry { lemma: "poščęditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "požaliti" => &[
        VerbDictionaryEntry { lemma: "požaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "poželati" => &[
        VerbDictionaryEntry { lemma: "poželati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "požirati" => &[
        VerbDictionaryEntry { lemma: "požirati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "požrtvovati" => &[
        VerbDictionaryEntry { lemma: "požrtvovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "požrěti" => &[
        VerbDictionaryEntry { lemma: "požrěti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "požędati" => &[
        VerbDictionaryEntry { lemma: "požędati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "požęti" => &[
        VerbDictionaryEntry { lemma: "požęti", addition: "(požne)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pracovati" => &[
        VerbDictionaryEntry { lemma: "pracovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "praktikovati" => &[
        VerbDictionaryEntry { lemma: "praktikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prati" => &[
        VerbDictionaryEntry { lemma: "prati", addition: "(pere)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "praviti" => &[
        VerbDictionaryEntry { lemma: "praviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prašćati" => &[
        VerbDictionaryEntry { lemma: "prašćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "preferovati" => &[
        VerbDictionaryEntry { lemma: "preferovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pribiti" => &[
        VerbDictionaryEntry { lemma: "pribiti", addition: "(pribije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pribivati" => &[
        VerbDictionaryEntry { lemma: "pribivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pribyti" => &[
        VerbDictionaryEntry { lemma: "pribyti", addition: "(pribųde)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pribyvati" => &[
        VerbDictionaryEntry { lemma: "pribyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pricěliti" => &[
        VerbDictionaryEntry { lemma: "pricěliti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pridati" => &[
        VerbDictionaryEntry { lemma: "pridati", addition: "(prida)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pridavati" => &[
        VerbDictionaryEntry { lemma: "pridavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pridumati" => &[
        VerbDictionaryEntry { lemma: "pridumati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pridělati" => &[
        VerbDictionaryEntry { lemma: "pridělati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "priděliti" => &[
        VerbDictionaryEntry { lemma: "priděliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "priděljati" => &[
        VerbDictionaryEntry { lemma: "priděljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "priględati" => &[
        VerbDictionaryEntry { lemma: "priględati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "priględěti" => &[
        VerbDictionaryEntry { lemma: "priględěti", addition: "(priględi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prignųti" => &[
        VerbDictionaryEntry { lemma: "prignųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prigotoviti" => &[
        VerbDictionaryEntry { lemma: "prigotoviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prigybati" => &[
        VerbDictionaryEntry { lemma: "prigybati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prihoditi" => &[
        VerbDictionaryEntry { lemma: "prihoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prijati" => &[
        VerbDictionaryEntry { lemma: "prijati", addition: "(+3)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prijdti" => &[
        VerbDictionaryEntry { lemma: "prijdti", addition: "(prijde; prišėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "prijehati" => &[
        VerbDictionaryEntry { lemma: "prijehati", addition: "(prijede)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "priježđati" => &[
        VerbDictionaryEntry { lemma: "priježđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prijmati" => &[
        VerbDictionaryEntry { lemma: "prijmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prijęti" => &[
        VerbDictionaryEntry { lemma: "prijęti", addition: "(prijme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prikazati" => &[
        VerbDictionaryEntry { lemma: "prikazati", addition: "(prikaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prikazyvati" => &[
        VerbDictionaryEntry { lemma: "prikazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prikladati" => &[
        VerbDictionaryEntry { lemma: "prikladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "priletěti" => &[
        VerbDictionaryEntry { lemma: "priletěti", addition: "(prilěti)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "priložiti" => &[
        VerbDictionaryEntry { lemma: "priložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prilěpiti" => &[
        VerbDictionaryEntry { lemma: "prilěpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prilětati" => &[
        VerbDictionaryEntry { lemma: "prilětati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prilųčati" => &[
        VerbDictionaryEntry { lemma: "prilųčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prilųčiti" => &[
        VerbDictionaryEntry { lemma: "prilųčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "priměniti" => &[
        VerbDictionaryEntry { lemma: "priměniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "priměnjati" => &[
        VerbDictionaryEntry { lemma: "priměnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "primětiti" => &[
        VerbDictionaryEntry { lemma: "primětiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "priměćati" => &[
        VerbDictionaryEntry { lemma: "priměćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prinaležati" => &[
        VerbDictionaryEntry { lemma: "prinaležati", addition: "(prinaleži)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prinesti" => &[
        VerbDictionaryEntry { lemma: "prinesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prinositi" => &[
        VerbDictionaryEntry { lemma: "prinositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prinuditi" => &[
        VerbDictionaryEntry { lemma: "prinuditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prinuđati" => &[
        VerbDictionaryEntry { lemma: "prinuđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pripadati" => &[
        VerbDictionaryEntry { lemma: "pripadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pripasti" => &[
        VerbDictionaryEntry { lemma: "pripasti", addition: "(pripade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pripinati" => &[
        VerbDictionaryEntry { lemma: "pripinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pripisati" => &[
        VerbDictionaryEntry { lemma: "pripisati", addition: "(pripiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pripisyvati" => &[
        VerbDictionaryEntry { lemma: "pripisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pripluti" => &[
        VerbDictionaryEntry { lemma: "pripluti", addition: "(priplove)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "priplyvati" => &[
        VerbDictionaryEntry { lemma: "priplyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "priplyvti" => &[
        VerbDictionaryEntry { lemma: "priplyvti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pripominati" => &[
        VerbDictionaryEntry { lemma: "pripominati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pripomněti" => &[
        VerbDictionaryEntry { lemma: "pripomněti", addition: "(pripomni)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pripraviti" => &[
        VerbDictionaryEntry { lemma: "pripraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pripravjati" => &[
        VerbDictionaryEntry { lemma: "pripravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pripustiti" => &[
        VerbDictionaryEntry { lemma: "pripustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pripušćati" => &[
        VerbDictionaryEntry { lemma: "pripušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pripęti" => &[
        VerbDictionaryEntry { lemma: "pripęti", addition: "(pripne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prisjediniti" => &[
        VerbDictionaryEntry { lemma: "prisjediniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prisjedinjati" => &[
        VerbDictionaryEntry { lemma: "prisjedinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prislati" => &[
        VerbDictionaryEntry { lemma: "prislati", addition: "(prišlje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prisposobiti" => &[
        VerbDictionaryEntry { lemma: "prisposobiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prisposobjati" => &[
        VerbDictionaryEntry { lemma: "prisposobjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prispěti" => &[
        VerbDictionaryEntry { lemma: "prispěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "prispěvati" => &[
        VerbDictionaryEntry { lemma: "prispěvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pristojati" => &[
        VerbDictionaryEntry { lemma: "pristojati", addition: "(pristoji)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pristrigati" => &[
        VerbDictionaryEntry { lemma: "pristrigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pristrigti" => &[
        VerbDictionaryEntry { lemma: "pristrigti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prisvajati" => &[
        VerbDictionaryEntry { lemma: "prisvajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prisvojiti" => &[
        VerbDictionaryEntry { lemma: "prisvojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prisylati" => &[
        VerbDictionaryEntry { lemma: "prisylati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prisęgati" => &[
        VerbDictionaryEntry { lemma: "prisęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prisęgnųti" => &[
        VerbDictionaryEntry { lemma: "prisęgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prisųditi" => &[
        VerbDictionaryEntry { lemma: "prisųditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prisųđati" => &[
        VerbDictionaryEntry { lemma: "prisųđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pritiskati" => &[
        VerbDictionaryEntry { lemma: "pritiskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pritisknųti" => &[
        VerbDictionaryEntry { lemma: "pritisknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pritęgati" => &[
        VerbDictionaryEntry { lemma: "pritęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pritęgnųti" => &[
        VerbDictionaryEntry { lemma: "pritęgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "priučati" => &[
        VerbDictionaryEntry { lemma: "priučati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "priučiti" => &[
        VerbDictionaryEntry { lemma: "priučiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "privabiti" => &[
        VerbDictionaryEntry { lemma: "privabiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "privabjati" => &[
        VerbDictionaryEntry { lemma: "privabjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "privesti" => &[
        VerbDictionaryEntry { lemma: "privesti", addition: "(privede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "privezti" => &[
        VerbDictionaryEntry { lemma: "privezti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "privitati" => &[
        VerbDictionaryEntry { lemma: "privitati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "privlåčivati" => &[
        VerbDictionaryEntry { lemma: "privlåčivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "privlěkati" => &[
        VerbDictionaryEntry { lemma: "privlěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "privlěkti" => &[
        VerbDictionaryEntry { lemma: "privlěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "privoditi" => &[
        VerbDictionaryEntry { lemma: "privoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "privoliti" => &[
        VerbDictionaryEntry { lemma: "privoliti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "privoljati" => &[
        VerbDictionaryEntry { lemma: "privoljati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "privoziti" => &[
        VerbDictionaryEntry { lemma: "privoziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "privykati" => &[
        VerbDictionaryEntry { lemma: "privykati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "privyknųti" => &[
        VerbDictionaryEntry { lemma: "privyknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "privęzati" => &[
        VerbDictionaryEntry { lemma: "privęzati", addition: "(privęže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "privęzyvati" => &[
        VerbDictionaryEntry { lemma: "privęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "priznati" => &[
        VerbDictionaryEntry { lemma: "priznati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "priznavati" => &[
        VerbDictionaryEntry { lemma: "priznavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prizvati" => &[
        VerbDictionaryEntry { lemma: "prizvati", addition: "(prizȯve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prizyvati" => &[
        VerbDictionaryEntry { lemma: "prizyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pričiniti" => &[
        VerbDictionaryEntry { lemma: "pričiniti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pričinjati" => &[
        VerbDictionaryEntry { lemma: "pričinjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prišiti" => &[
        VerbDictionaryEntry { lemma: "prišiti", addition: "(prišije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "probiti" => &[
        VerbDictionaryEntry { lemma: "probiti", addition: "(probije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "probivati" => &[
        VerbDictionaryEntry { lemma: "probivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "probovati" => &[
        VerbDictionaryEntry { lemma: "probovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "procitovati" => &[
        VerbDictionaryEntry { lemma: "procitovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "procvětati" => &[
        VerbDictionaryEntry { lemma: "procvětati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prodati" => &[
        VerbDictionaryEntry { lemma: "prodati", addition: "(proda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prodavati" => &[
        VerbDictionaryEntry { lemma: "prodavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prodiraviti" => &[
        VerbDictionaryEntry { lemma: "prodiraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prodiravjati" => &[
        VerbDictionaryEntry { lemma: "prodiravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "produkovati" => &[
        VerbDictionaryEntry { lemma: "produkovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "produmati" => &[
        VerbDictionaryEntry { lemma: "produmati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prodȯlžati" => &[
        VerbDictionaryEntry { lemma: "prodȯlžati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prodȯlžiti" => &[
        VerbDictionaryEntry { lemma: "prodȯlžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "profanovati" => &[
        VerbDictionaryEntry { lemma: "profanovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "profesionalizovati" => &[
        VerbDictionaryEntry { lemma: "profesionalizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "proganjati" => &[
        VerbDictionaryEntry { lemma: "proganjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "proglašati" => &[
        VerbDictionaryEntry { lemma: "proglašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "proglåsiti" => &[
        VerbDictionaryEntry { lemma: "proglåsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "programovati" => &[
        VerbDictionaryEntry { lemma: "programovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "progȯltati" => &[
        VerbDictionaryEntry { lemma: "progȯltati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "progȯltnųti" => &[
        VerbDictionaryEntry { lemma: "progȯltnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prohlåditi" => &[
        VerbDictionaryEntry { lemma: "prohlåditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prohoditi" => &[
        VerbDictionaryEntry { lemma: "prohoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "proigrati" => &[
        VerbDictionaryEntry { lemma: "proigrati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "proigryvati" => &[
        VerbDictionaryEntry { lemma: "proigryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "proiztekti" => &[
        VerbDictionaryEntry { lemma: "proiztekti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "proiztěkati" => &[
        VerbDictionaryEntry { lemma: "proiztěkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "proizvesti" => &[
        VerbDictionaryEntry { lemma: "proizvesti", addition: "(proizvede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "proizvoditi" => &[
        VerbDictionaryEntry { lemma: "proizvoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "projaviti" => &[
        VerbDictionaryEntry { lemma: "projaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "projavjati" => &[
        VerbDictionaryEntry { lemma: "projavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "projdti" => &[
        VerbDictionaryEntry { lemma: "projdti", addition: "(projde; prošėl)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "projehati" => &[
        VerbDictionaryEntry { lemma: "projehati", addition: "(projede)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "projektovati" => &[
        VerbDictionaryEntry { lemma: "projektovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "proježđati" => &[
        VerbDictionaryEntry { lemma: "proježđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "projmati" => &[
        VerbDictionaryEntry { lemma: "projmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "projęti" => &[
        VerbDictionaryEntry { lemma: "projęti", addition: "(projme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "proklęti" => &[
        VerbDictionaryEntry { lemma: "proklęti", addition: "(proklne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "proklęti", addition: "(proklne)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "prokontrolovati" => &[
        VerbDictionaryEntry { lemma: "prokontrolovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prokrijumčariti" => &[
        VerbDictionaryEntry { lemma: "prokrijumčariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prolamyvati" => &[
        VerbDictionaryEntry { lemma: "prolamyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "proliti" => &[
        VerbDictionaryEntry { lemma: "proliti", addition: "(prolije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prolivati" => &[
        VerbDictionaryEntry { lemma: "prolivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prolomiti" => &[
        VerbDictionaryEntry { lemma: "prolomiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prolězati" => &[
        VerbDictionaryEntry { lemma: "prolězati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prolězti" => &[
        VerbDictionaryEntry { lemma: "prolězti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pronevěriti" => &[
        VerbDictionaryEntry { lemma: "pronevěriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pronevěrjati" => &[
        VerbDictionaryEntry { lemma: "pronevěrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pronikati" => &[
        VerbDictionaryEntry { lemma: "pronikati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "proniknųti" => &[
        VerbDictionaryEntry { lemma: "proniknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "propadati" => &[
        VerbDictionaryEntry { lemma: "propadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "propagovati" => &[
        VerbDictionaryEntry { lemma: "propagovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "propasti" => &[
        VerbDictionaryEntry { lemma: "propasti", addition: "(propade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "propiti" => &[
        VerbDictionaryEntry { lemma: "propiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "propivati" => &[
        VerbDictionaryEntry { lemma: "propivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "proponovati" => &[
        VerbDictionaryEntry { lemma: "proponovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "propustiti" => &[
        VerbDictionaryEntry { lemma: "propustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "propušćati" => &[
        VerbDictionaryEntry { lemma: "propušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "proråstati" => &[
        VerbDictionaryEntry { lemma: "proråstati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prositi" => &[
        VerbDictionaryEntry { lemma: "prositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "proslaviti" => &[
        VerbDictionaryEntry { lemma: "proslaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "proslavjati" => &[
        VerbDictionaryEntry { lemma: "proslavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prospati" => &[
        VerbDictionaryEntry { lemma: "prospati", addition: "(prospi)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "prospati", addition: "(prospi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prostirati" => &[
        VerbDictionaryEntry { lemma: "prostirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prostiti" => &[
        VerbDictionaryEntry { lemma: "prostiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prostrěti" => &[
        VerbDictionaryEntry { lemma: "prostrěti", addition: "(prostre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prostŕti" => &[
        VerbDictionaryEntry { lemma: "prostŕti", addition: "(prostre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prosvětiti" => &[
        VerbDictionaryEntry { lemma: "prosvětiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prosvěćati" => &[
        VerbDictionaryEntry { lemma: "prosvěćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "protekti" => &[
        VerbDictionaryEntry { lemma: "protekti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "protestovati" => &[
        VerbDictionaryEntry { lemma: "protestovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "protivdějati" => &[
        VerbDictionaryEntry { lemma: "protivdějati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "protivrěčiti" => &[
        VerbDictionaryEntry { lemma: "protivrěčiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "protrěti" => &[
        VerbDictionaryEntry { lemma: "protrěti", addition: "(protre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "protěkati" => &[
        VerbDictionaryEntry { lemma: "protěkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "protŕti" => &[
        VerbDictionaryEntry { lemma: "protŕti", addition: "(protre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "provesti" => &[
        VerbDictionaryEntry { lemma: "provesti", addition: "(provede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "provoditi" => &[
        VerbDictionaryEntry { lemma: "provoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "provoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "provokovati" => &[
        VerbDictionaryEntry { lemma: "provokovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "provođati" => &[
        VerbDictionaryEntry { lemma: "provođati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prověriti" => &[
        VerbDictionaryEntry { lemma: "prověriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prověrjati" => &[
        VerbDictionaryEntry { lemma: "prověrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "provětriti" => &[
        VerbDictionaryEntry { lemma: "provětriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "provětrjati" => &[
        VerbDictionaryEntry { lemma: "provětrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "provŕgati" => &[
        VerbDictionaryEntry { lemma: "provŕgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "provŕgnųti" => &[
        VerbDictionaryEntry { lemma: "provŕgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "provŕtiti" => &[
        VerbDictionaryEntry { lemma: "provŕtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "provŕćati" => &[
        VerbDictionaryEntry { lemma: "provŕćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pročistiti" => &[
        VerbDictionaryEntry { lemma: "pročistiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pročitati" => &[
        VerbDictionaryEntry { lemma: "pročitati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pročišćati" => &[
        VerbDictionaryEntry { lemma: "pročišćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prošćati" => &[
        VerbDictionaryEntry { lemma: "prošćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "proživati" => &[
        VerbDictionaryEntry { lemma: "proživati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prskati" => &[
        VerbDictionaryEntry { lemma: "prskati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prsknųti" => &[
        VerbDictionaryEntry { lemma: "prsknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pråti" => &[
        VerbDictionaryEntry { lemma: "pråti", addition: "(poŕe)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pråzdnovati" => &[
        VerbDictionaryEntry { lemma: "pråzdnovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pråšiti" => &[
        VerbDictionaryEntry { lemma: "pråšiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pręsti" => &[
        VerbDictionaryEntry { lemma: "pręsti", addition: "(pręde)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "pręsti", addition: "(pręde)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěadresovati" => &[
        VerbDictionaryEntry { lemma: "prěadresovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěbudovati" => &[
        VerbDictionaryEntry { lemma: "prěbudovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěbudovyvati" => &[
        VerbDictionaryEntry { lemma: "prěbudovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěbyvati" => &[
        VerbDictionaryEntry { lemma: "prěbyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prěcěniti" => &[
        VerbDictionaryEntry { lemma: "prěcěniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěcěnjati" => &[
        VerbDictionaryEntry { lemma: "prěcěnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prědati" => &[
        VerbDictionaryEntry { lemma: "prědati", addition: "(prěda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prědavati" => &[
        VerbDictionaryEntry { lemma: "prědavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prědhoditi" => &[
        VerbDictionaryEntry { lemma: "prědhoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prědkladati" => &[
        VerbDictionaryEntry { lemma: "prědkladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prědlagati" => &[
        VerbDictionaryEntry { lemma: "prědlagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prědložiti" => &[
        VerbDictionaryEntry { lemma: "prědložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prědpisati" => &[
        VerbDictionaryEntry { lemma: "prědpisati", addition: "(prědpiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prědpisyvati" => &[
        VerbDictionaryEntry { lemma: "prědpisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prědplatiti" => &[
        VerbDictionaryEntry { lemma: "prědplatiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "prědplaćati" => &[
        VerbDictionaryEntry { lemma: "prědplaćati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prědpokladati" => &[
        VerbDictionaryEntry { lemma: "prědpokladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prědpolagati" => &[
        VerbDictionaryEntry { lemma: "prědpolagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prědpoložiti" => &[
        VerbDictionaryEntry { lemma: "prědpoložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prědpovědati" => &[
        VerbDictionaryEntry { lemma: "prědpovědati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prědpověděti" => &[
        VerbDictionaryEntry { lemma: "prědpověděti", addition: "(prědpově)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prědpočitati" => &[
        VerbDictionaryEntry { lemma: "prědpočitati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prědprijmati" => &[
        VerbDictionaryEntry { lemma: "prědprijmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prědprijęti" => &[
        VerbDictionaryEntry { lemma: "prědprijęti", addition: "(prědprijme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prědskazati" => &[
        VerbDictionaryEntry { lemma: "prědskazati", addition: "(prědskaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prědskazyvati" => &[
        VerbDictionaryEntry { lemma: "prědskazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prědstati" => &[
        VerbDictionaryEntry { lemma: "prědstati", addition: "(prědstane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "prědstavati" => &[
        VerbDictionaryEntry { lemma: "prědstavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prědstaviti" => &[
        VerbDictionaryEntry { lemma: "prědstaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prědstavjati" => &[
        VerbDictionaryEntry { lemma: "prědstavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prědsědati" => &[
        VerbDictionaryEntry { lemma: "prědsědati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prěduprěditi" => &[
        VerbDictionaryEntry { lemma: "prěduprěditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěduprěđati" => &[
        VerbDictionaryEntry { lemma: "prěduprěđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prědvidyvati" => &[
        VerbDictionaryEntry { lemma: "prědvidyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prědviděti" => &[
        VerbDictionaryEntry { lemma: "prědviděti", addition: "(prědvidi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prědvyšati" => &[
        VerbDictionaryEntry { lemma: "prědvyšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prědčuvati" => &[
        VerbDictionaryEntry { lemma: "prědčuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prědȯjdti" => &[
        VerbDictionaryEntry { lemma: "prědȯjdti", addition: "(prědȯjde; prědšėl)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěględati" => &[
        VerbDictionaryEntry { lemma: "prěględati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěględěti" => &[
        VerbDictionaryEntry { lemma: "prěględěti", addition: "(prěględi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěhoditi" => &[
        VerbDictionaryEntry { lemma: "prěhoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prěhytriti" => &[
        VerbDictionaryEntry { lemma: "prěhytriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěhytrjati" => &[
        VerbDictionaryEntry { lemma: "prěhytrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěimenovati" => &[
        VerbDictionaryEntry { lemma: "prěimenovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěimenovyvati" => &[
        VerbDictionaryEntry { lemma: "prěimenovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěinačati" => &[
        VerbDictionaryEntry { lemma: "prěinačati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěinačiti" => &[
        VerbDictionaryEntry { lemma: "prěinačiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prějdti" => &[
        VerbDictionaryEntry { lemma: "prějdti", addition: "(prějde; prěšėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "prějmati" => &[
        VerbDictionaryEntry { lemma: "prějmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prějęti" => &[
        VerbDictionaryEntry { lemma: "prějęti", addition: "(prějme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěkladati" => &[
        VerbDictionaryEntry { lemma: "prěkladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěključati" => &[
        VerbDictionaryEntry { lemma: "prěključati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěključiti" => &[
        VerbDictionaryEntry { lemma: "prěključiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěložiti" => &[
        VerbDictionaryEntry { lemma: "prěložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prělězati" => &[
        VerbDictionaryEntry { lemma: "prělězati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prělězti" => &[
        VerbDictionaryEntry { lemma: "prělězti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "prěmagati" => &[
        VerbDictionaryEntry { lemma: "prěmagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěmeblovati" => &[
        VerbDictionaryEntry { lemma: "prěmeblovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěmogti" => &[
        VerbDictionaryEntry { lemma: "prěmogti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěmotati" => &[
        VerbDictionaryEntry { lemma: "prěmotati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěmotyvati" => &[
        VerbDictionaryEntry { lemma: "prěmotyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěměniti" => &[
        VerbDictionaryEntry { lemma: "prěměniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěměnjati" => &[
        VerbDictionaryEntry { lemma: "prěměnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěměstiti" => &[
        VerbDictionaryEntry { lemma: "prěměstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěměšćati" => &[
        VerbDictionaryEntry { lemma: "prěměšćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěmȯlviti" => &[
        VerbDictionaryEntry { lemma: "prěmȯlviti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "prěmȯlvjati" => &[
        VerbDictionaryEntry { lemma: "prěmȯlvjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prěnapinati" => &[
        VerbDictionaryEntry { lemma: "prěnapinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěnapęti" => &[
        VerbDictionaryEntry { lemma: "prěnapęti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěnebrěgati" => &[
        VerbDictionaryEntry { lemma: "prěnebrěgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěnebrěgti" => &[
        VerbDictionaryEntry { lemma: "prěnebrěgti", addition: "(prěnebrěže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěnesti" => &[
        VerbDictionaryEntry { lemma: "prěnesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěnositi" => &[
        VerbDictionaryEntry { lemma: "prěnositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěnoćevati" => &[
        VerbDictionaryEntry { lemma: "prěnoćevati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "prěobraziti" => &[
        VerbDictionaryEntry { lemma: "prěobraziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěobraćati" => &[
        VerbDictionaryEntry { lemma: "prěobraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěobražati" => &[
        VerbDictionaryEntry { lemma: "prěobražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěobråtiti" => &[
        VerbDictionaryEntry { lemma: "prěobråtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěocěniti" => &[
        VerbDictionaryEntry { lemma: "prěocěniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěocěnjati" => &[
        VerbDictionaryEntry { lemma: "prěocěnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěpisati" => &[
        VerbDictionaryEntry { lemma: "prěpisati", addition: "(prěpiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěpisyvati" => &[
        VerbDictionaryEntry { lemma: "prěpisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěpluti" => &[
        VerbDictionaryEntry { lemma: "prěpluti", addition: "(prěplove)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěplyvati" => &[
        VerbDictionaryEntry { lemma: "prěplyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěplyvti" => &[
        VerbDictionaryEntry { lemma: "prěplyvti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěporųčati" => &[
        VerbDictionaryEntry { lemma: "prěporųčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěporųčiti" => &[
        VerbDictionaryEntry { lemma: "prěporųčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěpraviti" => &[
        VerbDictionaryEntry { lemma: "prěpraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěpravjati" => &[
        VerbDictionaryEntry { lemma: "prěpravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěprogramovati" => &[
        VerbDictionaryEntry { lemma: "prěprogramovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěrastati" => &[
        VerbDictionaryEntry { lemma: "prěrastati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěrvati" => &[
        VerbDictionaryEntry { lemma: "prěrvati", addition: "(prěrve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěryvati" => &[
        VerbDictionaryEntry { lemma: "prěryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěråsti" => &[
        VerbDictionaryEntry { lemma: "prěråsti", addition: "(prěråste)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěråzkazati" => &[
        VerbDictionaryEntry { lemma: "prěråzkazati", addition: "(prěskaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěråzkazyvati" => &[
        VerbDictionaryEntry { lemma: "prěråzkazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěrězati" => &[
        VerbDictionaryEntry { lemma: "prěrězati", addition: "(prěrěže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěseliti" => &[
        VerbDictionaryEntry { lemma: "prěseliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěseljati" => &[
        VerbDictionaryEntry { lemma: "prěseljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěslušati" => &[
        VerbDictionaryEntry { lemma: "prěslušati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěslušivati" => &[
        VerbDictionaryEntry { lemma: "prěslušivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěslědovati" => &[
        VerbDictionaryEntry { lemma: "prěslědovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěslěpiti" => &[
        VerbDictionaryEntry { lemma: "prěslěpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěsměriti" => &[
        VerbDictionaryEntry { lemma: "prěsměriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěsměrjati" => &[
        VerbDictionaryEntry { lemma: "prěsměrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěspati" => &[
        VerbDictionaryEntry { lemma: "prěspati", addition: "(prěspi)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "prěstati" => &[
        VerbDictionaryEntry { lemma: "prěstati", addition: "(prěstane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "prěstavati" => &[
        VerbDictionaryEntry { lemma: "prěstavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prěstaviti" => &[
        VerbDictionaryEntry { lemma: "prěstaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěstavjati" => &[
        VerbDictionaryEntry { lemma: "prěstavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěstigati" => &[
        VerbDictionaryEntry { lemma: "prěstigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěstignųti" => &[
        VerbDictionaryEntry { lemma: "prěstignųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěstrašiti" => &[
        VerbDictionaryEntry { lemma: "prěstrašiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěstųpati" => &[
        VerbDictionaryEntry { lemma: "prěstųpati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěstųpiti" => &[
        VerbDictionaryEntry { lemma: "prěstųpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěsunųti" => &[
        VerbDictionaryEntry { lemma: "prěsunųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěsuvati" => &[
        VerbDictionaryEntry { lemma: "prěsuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěsvęzati" => &[
        VerbDictionaryEntry { lemma: "prěsvęzati", addition: "(prěsvęže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěsvęzyvati" => &[
        VerbDictionaryEntry { lemma: "prěsvęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěsědati" => &[
        VerbDictionaryEntry { lemma: "prěsědati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prěsěkati" => &[
        VerbDictionaryEntry { lemma: "prěsěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěsěkti" => &[
        VerbDictionaryEntry { lemma: "prěsěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěsěsti" => &[
        VerbDictionaryEntry { lemma: "prěsěsti", addition: "(prěsěde)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "prětekti" => &[
        VerbDictionaryEntry { lemma: "prětekti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "prěti" => &[
        VerbDictionaryEntry { lemma: "prěti", addition: "(pre)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prětraviti" => &[
        VerbDictionaryEntry { lemma: "prětraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prětravjati" => &[
        VerbDictionaryEntry { lemma: "prětravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prětvarjati" => &[
        VerbDictionaryEntry { lemma: "prětvarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prětvoriti" => &[
        VerbDictionaryEntry { lemma: "prětvoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prětěkati" => &[
        VerbDictionaryEntry { lemma: "prětěkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prětŕpěti" => &[
        VerbDictionaryEntry { lemma: "prětŕpěti", addition: "(prětŕpi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prětȯlmačiti" => &[
        VerbDictionaryEntry { lemma: "prětȯlmačiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěuveličati" => &[
        VerbDictionaryEntry { lemma: "prěuveličati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěuveličiti" => &[
        VerbDictionaryEntry { lemma: "prěuveličiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěvariti" => &[
        VerbDictionaryEntry { lemma: "prěvariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěvarjati" => &[
        VerbDictionaryEntry { lemma: "prěvarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěvažati" => &[
        VerbDictionaryEntry { lemma: "prěvažati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "prěvažiti" => &[
        VerbDictionaryEntry { lemma: "prěvažiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "prěvesti" => &[
        VerbDictionaryEntry { lemma: "prěvesti", addition: "(prěvede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěvezti" => &[
        VerbDictionaryEntry { lemma: "prěvezti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěvladnųti" => &[
        VerbDictionaryEntry { lemma: "prěvladnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěvladyvati" => &[
        VerbDictionaryEntry { lemma: "prěvladyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěvoditi" => &[
        VerbDictionaryEntry { lemma: "prěvoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěvoziti" => &[
        VerbDictionaryEntry { lemma: "prěvoziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěvysiti" => &[
        VerbDictionaryEntry { lemma: "prěvysiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěvyšati" => &[
        VerbDictionaryEntry { lemma: "prěvyšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěvȯzhoditi" => &[
        VerbDictionaryEntry { lemma: "prěvȯzhoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěvȯzȯjdti" => &[
        VerbDictionaryEntry { lemma: "prěvȯzȯjdti", addition: "(prěvȯzȯjde; prěvȯzšėl)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prězirati" => &[
        VerbDictionaryEntry { lemma: "prězirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěznačiti" => &[
        VerbDictionaryEntry { lemma: "prěznačiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prězrěti" => &[
        VerbDictionaryEntry { lemma: "prězrěti", addition: "(prězri)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prězvati" => &[
        VerbDictionaryEntry { lemma: "prězvati", addition: "(prězȯve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prězyvati" => &[
        VerbDictionaryEntry { lemma: "prězyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěčiti" => &[
        VerbDictionaryEntry { lemma: "prěčiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěžiti" => &[
        VerbDictionaryEntry { lemma: "prěžiti", addition: "(prěžive)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prěživati" => &[
        VerbDictionaryEntry { lemma: "prěživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "prěžuvati" => &[
        VerbDictionaryEntry { lemma: "prěžuvati", addition: "(prěžuje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "prųžiti" => &[
        VerbDictionaryEntry { lemma: "prųžiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "psihovati" => &[
        VerbDictionaryEntry { lemma: "psihovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "publikovati" => &[
        VerbDictionaryEntry { lemma: "publikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "puhnųti" => &[
        VerbDictionaryEntry { lemma: "puhnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pustiti" => &[
        VerbDictionaryEntry { lemma: "pustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "pustošiti" => &[
        VerbDictionaryEntry { lemma: "pustošiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pustěti" => &[
        VerbDictionaryEntry { lemma: "pustěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pušiti" => &[
        VerbDictionaryEntry { lemma: "pušiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pušćati" => &[
        VerbDictionaryEntry { lemma: "pušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pyliti" => &[
        VerbDictionaryEntry { lemma: "pyliti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pytati" => &[
        VerbDictionaryEntry { lemma: "pytati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pěvati" => &[
        VerbDictionaryEntry { lemma: "pěvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pŕděti" => &[
        VerbDictionaryEntry { lemma: "pŕděti", addition: "(pŕdi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pŕti" => &[
        VerbDictionaryEntry { lemma: "pŕti", addition: "(pre)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pųditi" => &[
        VerbDictionaryEntry { lemma: "pųditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pųkati" => &[
        VerbDictionaryEntry { lemma: "pųkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pųknųti" => &[
        VerbDictionaryEntry { lemma: "pųknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "pųtovati" => &[
        VerbDictionaryEntry { lemma: "pųtovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pȯlniti" => &[
        VerbDictionaryEntry { lemma: "pȯlniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "pȯlzati" => &[
        VerbDictionaryEntry { lemma: "pȯlzati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "pȯlzti" => &[
        VerbDictionaryEntry { lemma: "pȯlzti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "racionalizovati" => &[
        VerbDictionaryEntry { lemma: "racionalizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "raditi" => &[
        VerbDictionaryEntry { lemma: "raditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "radovati" => &[
        VerbDictionaryEntry { lemma: "radovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "raniti" => &[
        VerbDictionaryEntry { lemma: "raniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ratifikovati" => &[
        VerbDictionaryEntry { lemma: "ratifikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "raziti" => &[
        VerbDictionaryEntry { lemma: "raziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "reagovati" => &[
        VerbDictionaryEntry { lemma: "reagovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "realizovati" => &[
        VerbDictionaryEntry { lemma: "realizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "recitovati" => &[
        VerbDictionaryEntry { lemma: "recitovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "redagovati" => &[
        VerbDictionaryEntry { lemma: "redagovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "redaktovati" => &[
        VerbDictionaryEntry { lemma: "redaktovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "redukovati" => &[
        VerbDictionaryEntry { lemma: "redukovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "reformovati" => &[
        VerbDictionaryEntry { lemma: "reformovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "regenerovati" => &[
        VerbDictionaryEntry { lemma: "regenerovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "registrovati" => &[
        VerbDictionaryEntry { lemma: "registrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "regulovati" => &[
        VerbDictionaryEntry { lemma: "regulovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "reklamovati" => &[
        VerbDictionaryEntry { lemma: "reklamovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "rekomendovati" => &[
        VerbDictionaryEntry { lemma: "rekomendovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "rekonstruovati" => &[
        VerbDictionaryEntry { lemma: "rekonstruovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "remontovati" => &[
        VerbDictionaryEntry { lemma: "remontovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "reorganizovati" => &[
        VerbDictionaryEntry { lemma: "reorganizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "reprezentovati" => &[
        VerbDictionaryEntry { lemma: "reprezentovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "reprodukovati" => &[
        VerbDictionaryEntry { lemma: "reprodukovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "restavrovati" => &[
        VerbDictionaryEntry { lemma: "restavrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "revidovati" => &[
        VerbDictionaryEntry { lemma: "revidovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "revti" => &[
        VerbDictionaryEntry { lemma: "revti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "rezervovati" => &[
        VerbDictionaryEntry { lemma: "rezervovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "rezumovati" => &[
        VerbDictionaryEntry { lemma: "rezumovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "rizikovati" => &[
        VerbDictionaryEntry { lemma: "rizikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "roditi" => &[
        VerbDictionaryEntry { lemma: "roditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "rokirovati" => &[
        VerbDictionaryEntry { lemma: "rokirovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true },
    ],
    "roptati" => &[
        VerbDictionaryEntry { lemma: "roptati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ruinovati" => &[
        VerbDictionaryEntry { lemma: "ruinovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ruměněti" => &[
        VerbDictionaryEntry { lemma: "ruměněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "rusifikovati" => &[
        VerbDictionaryEntry { lemma: "rusifikovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "rušiti" => &[
        VerbDictionaryEntry { lemma: "rušiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "rvati" => &[
        VerbDictionaryEntry { lemma: "rvati", addition: "(rve)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "rydati" => &[
        VerbDictionaryEntry { lemma: "rydati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "rygati" => &[
        VerbDictionaryEntry { lemma: "rygati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "rygnųti" => &[
        VerbDictionaryEntry { lemma: "rygnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "rymovati" => &[
        VerbDictionaryEntry { lemma: "rymovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "rysovati" => &[
        VerbDictionaryEntry { lemma: "rysovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ryti" => &[
        VerbDictionaryEntry { lemma: "ryti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "råbiti" => &[
        VerbDictionaryEntry { lemma: "råbiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råbotati" => &[
        VerbDictionaryEntry { lemma: "råbotati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "råsti" => &[
        VerbDictionaryEntry { lemma: "råsti", addition: "(råste)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "råvniti" => &[
        VerbDictionaryEntry { lemma: "råvniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzbirati" => &[
        VerbDictionaryEntry { lemma: "råzbirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzbiti" => &[
        VerbDictionaryEntry { lemma: "råzbiti", addition: "(råzbije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzbivati" => &[
        VerbDictionaryEntry { lemma: "råzbivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzbrati" => &[
        VerbDictionaryEntry { lemma: "råzbrati", addition: "(råzbere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzbuditi" => &[
        VerbDictionaryEntry { lemma: "råzbuditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzběsniti" => &[
        VerbDictionaryEntry { lemma: "råzběsniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzběsnjati" => &[
        VerbDictionaryEntry { lemma: "råzběsnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzcvěsti" => &[
        VerbDictionaryEntry { lemma: "råzcvěsti", addition: "(råzcvěte)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "råzcvětati" => &[
        VerbDictionaryEntry { lemma: "råzcvětati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "råzdati" => &[
        VerbDictionaryEntry { lemma: "råzdati", addition: "(råzda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzdavati" => &[
        VerbDictionaryEntry { lemma: "råzdavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzdirati" => &[
        VerbDictionaryEntry { lemma: "råzdirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzdražniti" => &[
        VerbDictionaryEntry { lemma: "råzdražniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzdražnjati" => &[
        VerbDictionaryEntry { lemma: "råzdražnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzdrobiti" => &[
        VerbDictionaryEntry { lemma: "råzdrobiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzdrobjati" => &[
        VerbDictionaryEntry { lemma: "råzdrobjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzdrěti" => &[
        VerbDictionaryEntry { lemma: "råzdrěti", addition: "(råzdere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzdvajati" => &[
        VerbDictionaryEntry { lemma: "råzdvajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzdvojiti" => &[
        VerbDictionaryEntry { lemma: "råzdvojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzděliti" => &[
        VerbDictionaryEntry { lemma: "råzděliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzděljati" => &[
        VerbDictionaryEntry { lemma: "råzděljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzděti" => &[
        VerbDictionaryEntry { lemma: "råzděti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzděvati" => &[
        VerbDictionaryEntry { lemma: "råzděvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzdŕti" => &[
        VerbDictionaryEntry { lemma: "råzdŕti", addition: "(råzdere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzdųti" => &[
        VerbDictionaryEntry { lemma: "råzdųti", addition: "(råzdme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzdųvati" => &[
        VerbDictionaryEntry { lemma: "råzdųvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzdȯlbati" => &[
        VerbDictionaryEntry { lemma: "råzdȯlbati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzdȯlbti" => &[
        VerbDictionaryEntry { lemma: "råzdȯlbti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzgadati" => &[
        VerbDictionaryEntry { lemma: "råzgadati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzgadyvati" => &[
        VerbDictionaryEntry { lemma: "råzgadyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzganjati" => &[
        VerbDictionaryEntry { lemma: "råzganjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzgladiti" => &[
        VerbDictionaryEntry { lemma: "råzgladiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzglađati" => &[
        VerbDictionaryEntry { lemma: "råzglađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzglašati" => &[
        VerbDictionaryEntry { lemma: "råzglašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzglåsiti" => &[
        VerbDictionaryEntry { lemma: "råzglåsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzględati" => &[
        VerbDictionaryEntry { lemma: "råzględati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzględěti" => &[
        VerbDictionaryEntry { lemma: "råzględěti", addition: "(råzględi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzgnati" => &[
        VerbDictionaryEntry { lemma: "råzgnati", addition: "(råzgone)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzgněvati" => &[
        VerbDictionaryEntry { lemma: "råzgněvati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzgovarjati" => &[
        VerbDictionaryEntry { lemma: "råzgovarjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "råzgrabiti" => &[
        VerbDictionaryEntry { lemma: "råzgrabiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzgrabjati" => &[
        VerbDictionaryEntry { lemma: "råzgrabjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzgraničati" => &[
        VerbDictionaryEntry { lemma: "råzgraničati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzgraničiti" => &[
        VerbDictionaryEntry { lemma: "råzgraničiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzgrađati" => &[
        VerbDictionaryEntry { lemma: "råzgrađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzgromiti" => &[
        VerbDictionaryEntry { lemma: "råzgromiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzgryzati" => &[
        VerbDictionaryEntry { lemma: "råzgryzati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzgryzti" => &[
        VerbDictionaryEntry { lemma: "råzgryzti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzgråditi" => &[
        VerbDictionaryEntry { lemma: "råzgråditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzgrěti" => &[
        VerbDictionaryEntry { lemma: "råzgrěti", addition: "(råzgrěje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzgrěvati" => &[
        VerbDictionaryEntry { lemma: "råzgrěvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzigrati" => &[
        VerbDictionaryEntry { lemma: "råzigrati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzigryvati" => &[
        VerbDictionaryEntry { lemma: "råzigryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råziskati" => &[
        VerbDictionaryEntry { lemma: "råziskati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råziskyvati" => &[
        VerbDictionaryEntry { lemma: "råziskyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzjariti" => &[
        VerbDictionaryEntry { lemma: "råzjariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzjarjati" => &[
        VerbDictionaryEntry { lemma: "råzjarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzjasniti" => &[
        VerbDictionaryEntry { lemma: "råzjasniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzjasnjati" => &[
        VerbDictionaryEntry { lemma: "råzjasnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzjebati" => &[
        VerbDictionaryEntry { lemma: "råzjebati", addition: "(råzjebe)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzjebyvati" => &[
        VerbDictionaryEntry { lemma: "råzjebyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzjedati" => &[
        VerbDictionaryEntry { lemma: "råzjedati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzjediniti" => &[
        VerbDictionaryEntry { lemma: "råzjediniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzjedinjati" => &[
        VerbDictionaryEntry { lemma: "råzjedinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzjesti" => &[
        VerbDictionaryEntry { lemma: "råzjesti", addition: "(råzje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzkalati" => &[
        VerbDictionaryEntry { lemma: "råzkalati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzkazati" => &[
        VerbDictionaryEntry { lemma: "råzkazati", addition: "(råzkaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzkazyvati" => &[
        VerbDictionaryEntry { lemma: "råzkazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzkladati" => &[
        VerbDictionaryEntry { lemma: "råzkladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzklejati" => &[
        VerbDictionaryEntry { lemma: "råzklejati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzklejiti" => &[
        VerbDictionaryEntry { lemma: "råzklejiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzklåti" => &[
        VerbDictionaryEntry { lemma: "råzklåti", addition: "(råzkolje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzkodovati" => &[
        VerbDictionaryEntry { lemma: "råzkodovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzkopati" => &[
        VerbDictionaryEntry { lemma: "råzkopati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzkopyvati" => &[
        VerbDictionaryEntry { lemma: "råzkopyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzkovati" => &[
        VerbDictionaryEntry { lemma: "råzkovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzkovyvati" => &[
        VerbDictionaryEntry { lemma: "råzkovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzkrajati" => &[
        VerbDictionaryEntry { lemma: "råzkrajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzkrojiti" => &[
        VerbDictionaryEntry { lemma: "råzkrojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzkryti" => &[
        VerbDictionaryEntry { lemma: "råzkryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzkryvati" => &[
        VerbDictionaryEntry { lemma: "råzkryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzkrųtiti" => &[
        VerbDictionaryEntry { lemma: "råzkrųtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzkrųćati" => &[
        VerbDictionaryEntry { lemma: "råzkrųćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzkrȯšiti" => &[
        VerbDictionaryEntry { lemma: "råzkrȯšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzkvartirovati" => &[
        VerbDictionaryEntry { lemma: "råzkvartirovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzkvartirovyvati" => &[
        VerbDictionaryEntry { lemma: "råzkvartirovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzkydati" => &[
        VerbDictionaryEntry { lemma: "råzkydati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzkydyvati" => &[
        VerbDictionaryEntry { lemma: "råzkydyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzladiti" => &[
        VerbDictionaryEntry { lemma: "råzladiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzlagati" => &[
        VerbDictionaryEntry { lemma: "råzlagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzlamyvati" => &[
        VerbDictionaryEntry { lemma: "råzlamyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzlađati" => &[
        VerbDictionaryEntry { lemma: "råzlađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzliti" => &[
        VerbDictionaryEntry { lemma: "råzliti", addition: "(råzlije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzlivati" => &[
        VerbDictionaryEntry { lemma: "råzlivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzličati" => &[
        VerbDictionaryEntry { lemma: "råzličati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzličiti" => &[
        VerbDictionaryEntry { lemma: "råzličiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzlomiti" => &[
        VerbDictionaryEntry { lemma: "råzlomiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzložiti" => &[
        VerbDictionaryEntry { lemma: "råzložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzlųčati" => &[
        VerbDictionaryEntry { lemma: "råzlųčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzlųčiti" => &[
        VerbDictionaryEntry { lemma: "råzlųčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzmazati" => &[
        VerbDictionaryEntry { lemma: "råzmazati", addition: "(råzmaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzmazyvati" => &[
        VerbDictionaryEntry { lemma: "råzmazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzmesti" => &[
        VerbDictionaryEntry { lemma: "råzmesti", addition: "(råzmete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzmetati" => &[
        VerbDictionaryEntry { lemma: "råzmetati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzmetyvati" => &[
        VerbDictionaryEntry { lemma: "råzmetyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzmlåtiti" => &[
        VerbDictionaryEntry { lemma: "råzmlåtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzmlěti" => &[
        VerbDictionaryEntry { lemma: "råzmlěti", addition: "(råzmelje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzmnažati" => &[
        VerbDictionaryEntry { lemma: "råzmnažati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzmnožiti" => &[
        VerbDictionaryEntry { lemma: "råzmnožiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzmokati" => &[
        VerbDictionaryEntry { lemma: "råzmokati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "råzmoknųti" => &[
        VerbDictionaryEntry { lemma: "råzmoknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "råzmontovati" => &[
        VerbDictionaryEntry { lemma: "råzmontovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzmotati" => &[
        VerbDictionaryEntry { lemma: "råzmotati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzmražati" => &[
        VerbDictionaryEntry { lemma: "råzmražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzmråziti" => &[
        VerbDictionaryEntry { lemma: "råzmråziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzmysliti" => &[
        VerbDictionaryEntry { lemma: "råzmysliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzmysljati" => &[
        VerbDictionaryEntry { lemma: "råzmysljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzmyti" => &[
        VerbDictionaryEntry { lemma: "råzmyti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzmyvati" => &[
        VerbDictionaryEntry { lemma: "råzmyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzmękčati" => &[
        VerbDictionaryEntry { lemma: "råzmękčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzmękčiti" => &[
        VerbDictionaryEntry { lemma: "råzmękčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzmęti" => &[
        VerbDictionaryEntry { lemma: "råzmęti", addition: "(råzmne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzměniti" => &[
        VerbDictionaryEntry { lemma: "råzměniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzměnjati" => &[
        VerbDictionaryEntry { lemma: "råzměnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzměriti" => &[
        VerbDictionaryEntry { lemma: "råzměriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzměrjati" => &[
        VerbDictionaryEntry { lemma: "råzměrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzměstiti" => &[
        VerbDictionaryEntry { lemma: "råzměstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzmětati" => &[
        VerbDictionaryEntry { lemma: "råzmětati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzměšati" => &[
        VerbDictionaryEntry { lemma: "råzměšati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzměšivati" => &[
        VerbDictionaryEntry { lemma: "råzměšivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzměšćati" => &[
        VerbDictionaryEntry { lemma: "råzměšćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råznesti" => &[
        VerbDictionaryEntry { lemma: "råznesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råznositi" => &[
        VerbDictionaryEntry { lemma: "råznositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzorati" => &[
        VerbDictionaryEntry { lemma: "råzorati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzoriti" => &[
        VerbDictionaryEntry { lemma: "råzoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzorjati" => &[
        VerbDictionaryEntry { lemma: "råzorjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzorųžati" => &[
        VerbDictionaryEntry { lemma: "råzorųžati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzorųžiti" => &[
        VerbDictionaryEntry { lemma: "råzorųžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzočarovati" => &[
        VerbDictionaryEntry { lemma: "råzočarovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzočarovyvati" => &[
        VerbDictionaryEntry { lemma: "råzočarovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzpakovati" => &[
        VerbDictionaryEntry { lemma: "råzpakovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzpakovyvati" => &[
        VerbDictionaryEntry { lemma: "råzpakovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzpaliti" => &[
        VerbDictionaryEntry { lemma: "råzpaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzpaljati" => &[
        VerbDictionaryEntry { lemma: "råzpaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzparjati" => &[
        VerbDictionaryEntry { lemma: "råzparjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzpečatati" => &[
        VerbDictionaryEntry { lemma: "råzpečatati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzpečatyvati" => &[
        VerbDictionaryEntry { lemma: "råzpečatyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzpinati" => &[
        VerbDictionaryEntry { lemma: "råzpinati", addition: "(råzpne)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzplesti" => &[
        VerbDictionaryEntry { lemma: "råzplesti", addition: "(råzplete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzpletati" => &[
        VerbDictionaryEntry { lemma: "råzpletati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzporęditi" => &[
        VerbDictionaryEntry { lemma: "råzporęditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzporęđati" => &[
        VerbDictionaryEntry { lemma: "råzporęđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzpoznati" => &[
        VerbDictionaryEntry { lemma: "råzpoznati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzpoznavati" => &[
        VerbDictionaryEntry { lemma: "råzpoznavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzprašati" => &[
        VerbDictionaryEntry { lemma: "råzprašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzprodati" => &[
        VerbDictionaryEntry { lemma: "råzprodati", addition: "(råzproda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzprodavati" => &[
        VerbDictionaryEntry { lemma: "råzprodavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzprostirati" => &[
        VerbDictionaryEntry { lemma: "råzprostirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzprostranjati" => &[
        VerbDictionaryEntry { lemma: "råzprostranjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzprostråniti" => &[
        VerbDictionaryEntry { lemma: "råzprostråniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzprostrěti" => &[
        VerbDictionaryEntry { lemma: "råzprostrěti", addition: "(råzprostre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzprostŕti" => &[
        VerbDictionaryEntry { lemma: "råzprostŕti", addition: "(råzprostre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzpråti" => &[
        VerbDictionaryEntry { lemma: "råzpråti", addition: "(råzpoŕe)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzpråšiti" => &[
        VerbDictionaryEntry { lemma: "råzpråšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzpustiti" => &[
        VerbDictionaryEntry { lemma: "råzpustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzpušćati" => &[
        VerbDictionaryEntry { lemma: "råzpušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzpytati" => &[
        VerbDictionaryEntry { lemma: "råzpytati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzpytyvati" => &[
        VerbDictionaryEntry { lemma: "råzpytyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzpęti" => &[
        VerbDictionaryEntry { lemma: "råzpęti", addition: "(råzpne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzpěniti" => &[
        VerbDictionaryEntry { lemma: "råzpěniti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "råzradovati" => &[
        VerbDictionaryEntry { lemma: "råzradovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzraznjati" => &[
        VerbDictionaryEntry { lemma: "råzraznjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzrušati" => &[
        VerbDictionaryEntry { lemma: "råzrušati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzrušiti" => &[
        VerbDictionaryEntry { lemma: "råzrušiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzrvati" => &[
        VerbDictionaryEntry { lemma: "råzrvati", addition: "(råzrve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzryti" => &[
        VerbDictionaryEntry { lemma: "råzryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzryvati" => &[
        VerbDictionaryEntry { lemma: "råzryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzråbotati" => &[
        VerbDictionaryEntry { lemma: "råzråbotati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzråbotyvati" => &[
        VerbDictionaryEntry { lemma: "råzråbotyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzråzniti" => &[
        VerbDictionaryEntry { lemma: "råzråzniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzrězati" => &[
        VerbDictionaryEntry { lemma: "råzrězati", addition: "(råzrěže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzrěšati" => &[
        VerbDictionaryEntry { lemma: "råzrěšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzrěšiti" => &[
        VerbDictionaryEntry { lemma: "råzrěšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzrųbati" => &[
        VerbDictionaryEntry { lemma: "råzrųbati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzrųbyvati" => &[
        VerbDictionaryEntry { lemma: "råzrųbyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzsaditi" => &[
        VerbDictionaryEntry { lemma: "råzsaditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzsađati" => &[
        VerbDictionaryEntry { lemma: "råzsađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzsedlati" => &[
        VerbDictionaryEntry { lemma: "råzsedlati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzsedlyvati" => &[
        VerbDictionaryEntry { lemma: "råzsedlyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzslati" => &[
        VerbDictionaryEntry { lemma: "råzslati", addition: "(råzšlje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzslěditi" => &[
        VerbDictionaryEntry { lemma: "råzslěditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzslědovati" => &[
        VerbDictionaryEntry { lemma: "råzslědovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzsmatrjati" => &[
        VerbDictionaryEntry { lemma: "råzsmatrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzsmotriti" => &[
        VerbDictionaryEntry { lemma: "råzsmotriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzsměšati" => &[
        VerbDictionaryEntry { lemma: "råzsměšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzsměšiti" => &[
        VerbDictionaryEntry { lemma: "råzsměšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzstaviti" => &[
        VerbDictionaryEntry { lemma: "råzstaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzstavjati" => &[
        VerbDictionaryEntry { lemma: "råzstavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzsteliti" => &[
        VerbDictionaryEntry { lemma: "råzsteliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzstrajati" => &[
        VerbDictionaryEntry { lemma: "råzstrajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzstrojiti" => &[
        VerbDictionaryEntry { lemma: "råzstrojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzstrěliti" => &[
        VerbDictionaryEntry { lemma: "råzstrěliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzstrěljati" => &[
        VerbDictionaryEntry { lemma: "råzstrěljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzsvirěpěti" => &[
        VerbDictionaryEntry { lemma: "råzsvirěpěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "råzsylati" => &[
        VerbDictionaryEntry { lemma: "råzsylati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzsypati" => &[
        VerbDictionaryEntry { lemma: "råzsypati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzsypyvati" => &[
        VerbDictionaryEntry { lemma: "råzsypyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzsčitati" => &[
        VerbDictionaryEntry { lemma: "råzsčitati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzsčityvati" => &[
        VerbDictionaryEntry { lemma: "råzsčityvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzsějati" => &[
        VerbDictionaryEntry { lemma: "råzsějati", addition: "(råzsěje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzsějivati" => &[
        VerbDictionaryEntry { lemma: "råzsějivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzsŕditi" => &[
        VerbDictionaryEntry { lemma: "råzsŕditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzsųditi" => &[
        VerbDictionaryEntry { lemma: "råzsųditi", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "råzsųđati" => &[
        VerbDictionaryEntry { lemma: "råzsųđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "råztajati" => &[
        VerbDictionaryEntry { lemma: "råztajati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råztapjati" => &[
        VerbDictionaryEntry { lemma: "råztapjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råztirati" => &[
        VerbDictionaryEntry { lemma: "råztirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råztopiti" => &[
        VerbDictionaryEntry { lemma: "råztopiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råztratiti" => &[
        VerbDictionaryEntry { lemma: "råztratiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råztraćati" => &[
        VerbDictionaryEntry { lemma: "råztraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råztrgati" => &[
        VerbDictionaryEntry { lemma: "råztrgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råztrgnųti" => &[
        VerbDictionaryEntry { lemma: "råztrgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råztrojiti" => &[
        VerbDictionaryEntry { lemma: "råztrojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råztrěskati" => &[
        VerbDictionaryEntry { lemma: "råztrěskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råztrěsknųti" => &[
        VerbDictionaryEntry { lemma: "råztrěsknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råztrěti" => &[
        VerbDictionaryEntry { lemma: "råztrěti", addition: "(råztre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råztrųbiti" => &[
        VerbDictionaryEntry { lemma: "råztrųbiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råztvarjati" => &[
        VerbDictionaryEntry { lemma: "råztvarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råztvoriti" => &[
        VerbDictionaryEntry { lemma: "råztvoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råztęgati" => &[
        VerbDictionaryEntry { lemma: "råztęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råztęgnųti" => &[
        VerbDictionaryEntry { lemma: "råztęgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råztŕti" => &[
        VerbDictionaryEntry { lemma: "råztŕti", addition: "(råztre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råztȯlkati" => &[
        VerbDictionaryEntry { lemma: "råztȯlkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råztȯlkti" => &[
        VerbDictionaryEntry { lemma: "råztȯlkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råztȯlstěti" => &[
        VerbDictionaryEntry { lemma: "råztȯlstěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "råztȯptati" => &[
        VerbDictionaryEntry { lemma: "råztȯptati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råztȯptyvati" => &[
        VerbDictionaryEntry { lemma: "råztȯptyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzumovati" => &[
        VerbDictionaryEntry { lemma: "råzumovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "råzuměti" => &[
        VerbDictionaryEntry { lemma: "råzuměti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzvaliti" => &[
        VerbDictionaryEntry { lemma: "råzvaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzvaljati" => &[
        VerbDictionaryEntry { lemma: "råzvaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzvažati" => &[
        VerbDictionaryEntry { lemma: "råzvažati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzvažiti" => &[
        VerbDictionaryEntry { lemma: "råzvažiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzveseliti" => &[
        VerbDictionaryEntry { lemma: "råzveseliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzveseljati" => &[
        VerbDictionaryEntry { lemma: "råzveseljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzvesti" => &[
        VerbDictionaryEntry { lemma: "råzvesti", addition: "(råzvede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "råzvesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzvezti" => &[
        VerbDictionaryEntry { lemma: "råzvezti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzviti" => &[
        VerbDictionaryEntry { lemma: "råzviti", addition: "(råzvije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzvivati" => &[
        VerbDictionaryEntry { lemma: "råzvivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzvoditi" => &[
        VerbDictionaryEntry { lemma: "råzvoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzvoziti" => &[
        VerbDictionaryEntry { lemma: "råzvoziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzvęzati" => &[
        VerbDictionaryEntry { lemma: "råzvęzati", addition: "(råzvęže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzvęzyvati" => &[
        VerbDictionaryEntry { lemma: "råzvęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzvědati" => &[
        VerbDictionaryEntry { lemma: "råzvědati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzvědyvati" => &[
        VerbDictionaryEntry { lemma: "råzvědyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzčesati" => &[
        VerbDictionaryEntry { lemma: "råzčesati", addition: "(råzčeše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzčistiti" => &[
        VerbDictionaryEntry { lemma: "råzčistiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzčišćati" => &[
        VerbDictionaryEntry { lemma: "råzčišćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzčuliti" => &[
        VerbDictionaryEntry { lemma: "råzčuliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzčuljati" => &[
        VerbDictionaryEntry { lemma: "råzčuljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzšifrovati" => &[
        VerbDictionaryEntry { lemma: "råzšifrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzširiti" => &[
        VerbDictionaryEntry { lemma: "råzširiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzširjati" => &[
        VerbDictionaryEntry { lemma: "råzširjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzšnurovati" => &[
        VerbDictionaryEntry { lemma: "råzšnurovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzšnurovyvati" => &[
        VerbDictionaryEntry { lemma: "råzšnurovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzščepiti" => &[
        VerbDictionaryEntry { lemma: "råzščepiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzščepjati" => &[
        VerbDictionaryEntry { lemma: "råzščepjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzžegti" => &[
        VerbDictionaryEntry { lemma: "råzžegti", addition: "(råzžže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzžerati" => &[
        VerbDictionaryEntry { lemma: "råzžerati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzžigati" => &[
        VerbDictionaryEntry { lemma: "råzžigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "råzžrěti" => &[
        VerbDictionaryEntry { lemma: "råzžrěti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "råzžuvati" => &[
        VerbDictionaryEntry { lemma: "råzžuvati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "rěkti" => &[
        VerbDictionaryEntry { lemma: "rěkti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "rězati" => &[
        VerbDictionaryEntry { lemma: "rězati", addition: "(rěže)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "rěšati" => &[
        VerbDictionaryEntry { lemma: "rěšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "rěšiti" => &[
        VerbDictionaryEntry { lemma: "rěšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "rųbati" => &[
        VerbDictionaryEntry { lemma: "rųbati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "rųkopleskati" => &[
        VerbDictionaryEntry { lemma: "rųkopleskati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "rųkoplesknųti" => &[
        VerbDictionaryEntry { lemma: "rųkoplesknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "rųkovati" => &[
        VerbDictionaryEntry { lemma: "rųkovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "rųkovoditi" => &[
        VerbDictionaryEntry { lemma: "rųkovoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sabotovati" => &[
        VerbDictionaryEntry { lemma: "sabotovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "saditi" => &[
        VerbDictionaryEntry { lemma: "saditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "samoubiti" => &[
        VerbDictionaryEntry { lemma: "samoubiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "sbirati" => &[
        VerbDictionaryEntry { lemma: "sbirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sbudovati" => &[
        VerbDictionaryEntry { lemma: "sbudovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "scati" => &[
        VerbDictionaryEntry { lemma: "scati", addition: "(sci)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sdeformovati" => &[
        VerbDictionaryEntry { lemma: "sdeformovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sdirati" => &[
        VerbDictionaryEntry { lemma: "sdirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sdrěmnųti" => &[
        VerbDictionaryEntry { lemma: "sdrěmnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "sdrěti" => &[
        VerbDictionaryEntry { lemma: "sdrěti", addition: "(sdre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sdyhati" => &[
        VerbDictionaryEntry { lemma: "sdyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sdělati" => &[
        VerbDictionaryEntry { lemma: "sdělati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sdŕti" => &[
        VerbDictionaryEntry { lemma: "sdŕti", addition: "(sdre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sdȯhnųti" => &[
        VerbDictionaryEntry { lemma: "sdȯhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "sebesmŕtiti" => &[
        VerbDictionaryEntry { lemma: "sebesmŕtiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sedlati" => &[
        VerbDictionaryEntry { lemma: "sedlati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sgnesti" => &[
        VerbDictionaryEntry { lemma: "sgnesti", addition: "(sgnete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sgnųti" => &[
        VerbDictionaryEntry { lemma: "sgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sgrěšiti" => &[
        VerbDictionaryEntry { lemma: "sgrěšiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "sgybati" => &[
        VerbDictionaryEntry { lemma: "sgybati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "shoditi" => &[
        VerbDictionaryEntry { lemma: "shoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "shovati" => &[
        VerbDictionaryEntry { lemma: "shovati", addition: "(shovaje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "shranjati" => &[
        VerbDictionaryEntry { lemma: "shranjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "shråniti" => &[
        VerbDictionaryEntry { lemma: "shråniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "shudnųti" => &[
        VerbDictionaryEntry { lemma: "shudnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "shvatiti" => &[
        VerbDictionaryEntry { lemma: "shvatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "signalizovati" => &[
        VerbDictionaryEntry { lemma: "signalizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sikati" => &[
        VerbDictionaryEntry { lemma: "sikati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "simbolizovati" => &[
        VerbDictionaryEntry { lemma: "simbolizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "simulovati" => &[
        VerbDictionaryEntry { lemma: "simulovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "sinhronizovati" => &[
        VerbDictionaryEntry { lemma: "sinhronizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "siněti" => &[
        VerbDictionaryEntry { lemma: "siněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sirotěti" => &[
        VerbDictionaryEntry { lemma: "sirotěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sivěti" => &[
        VerbDictionaryEntry { lemma: "sivěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sjedati" => &[
        VerbDictionaryEntry { lemma: "sjedati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sjediniti" => &[
        VerbDictionaryEntry { lemma: "sjediniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sjedinjati" => &[
        VerbDictionaryEntry { lemma: "sjedinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sjesti" => &[
        VerbDictionaryEntry { lemma: "sjesti", addition: "(sje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sjęti" => &[
        VerbDictionaryEntry { lemma: "sjęti", addition: "(sȯjme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "skakati" => &[
        VerbDictionaryEntry { lemma: "skakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "skanovati" => &[
        VerbDictionaryEntry { lemma: "skanovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "skazati" => &[
        VerbDictionaryEntry { lemma: "skazati", addition: "(skaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "skladati" => &[
        VerbDictionaryEntry { lemma: "skladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "skladovati" => &[
        VerbDictionaryEntry { lemma: "skladovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sklanjati" => &[
        VerbDictionaryEntry { lemma: "sklanjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sklejiti" => &[
        VerbDictionaryEntry { lemma: "sklejiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "skloniti" => &[
        VerbDictionaryEntry { lemma: "skloniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "skombinovati" => &[
        VerbDictionaryEntry { lemma: "skombinovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "skompensovati" => &[
        VerbDictionaryEntry { lemma: "skompensovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "skomplikovati" => &[
        VerbDictionaryEntry { lemma: "skomplikovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "skonfiskovati" => &[
        VerbDictionaryEntry { lemma: "skonfiskovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "skončiti" => &[
        VerbDictionaryEntry { lemma: "skončiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "skopiti" => &[
        VerbDictionaryEntry { lemma: "skopiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "skočiti" => &[
        VerbDictionaryEntry { lemma: "skočiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "skraćati" => &[
        VerbDictionaryEntry { lemma: "skraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "skrběti" => &[
        VerbDictionaryEntry { lemma: "skrběti", addition: "(skrbi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "skripnųti" => &[
        VerbDictionaryEntry { lemma: "skripnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "skripěti" => &[
        VerbDictionaryEntry { lemma: "skripěti", addition: "(skripi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "skriviti" => &[
        VerbDictionaryEntry { lemma: "skriviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "skryti" => &[
        VerbDictionaryEntry { lemma: "skryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "skryvati" => &[
        VerbDictionaryEntry { lemma: "skryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "skråtiti" => &[
        VerbDictionaryEntry { lemma: "skråtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "skubnųti" => &[
        VerbDictionaryEntry { lemma: "skubnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "skubti" => &[
        VerbDictionaryEntry { lemma: "skubti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "skuhati" => &[
        VerbDictionaryEntry { lemma: "skuhati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "skvrčati" => &[
        VerbDictionaryEntry { lemma: "skvrčati", addition: "(skvrči)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "slaběti" => &[
        VerbDictionaryEntry { lemma: "slaběti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "slati" => &[
        VerbDictionaryEntry { lemma: "slati", addition: "(šlje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "slaviti" => &[
        VerbDictionaryEntry { lemma: "slaviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "slgati" => &[
        VerbDictionaryEntry { lemma: "slgati", addition: "(slže)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "sliti" => &[
        VerbDictionaryEntry { lemma: "sliti", addition: "(slije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "slivati" => &[
        VerbDictionaryEntry { lemma: "slivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "slizgati" => &[
        VerbDictionaryEntry { lemma: "slizgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "slizgnųti" => &[
        VerbDictionaryEntry { lemma: "slizgnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "slomiti" => &[
        VerbDictionaryEntry { lemma: "slomiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "slovjanizovati" => &[
        VerbDictionaryEntry { lemma: "slovjanizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "složiti" => &[
        VerbDictionaryEntry { lemma: "složiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "slušati" => &[
        VerbDictionaryEntry { lemma: "slušati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "služiti" => &[
        VerbDictionaryEntry { lemma: "služiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "slynųti" => &[
        VerbDictionaryEntry { lemma: "slynųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "slyti" => &[
        VerbDictionaryEntry { lemma: "slyti", addition: "(slyne)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "slyšati" => &[
        VerbDictionaryEntry { lemma: "slyšati", addition: "(slyši)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "slåditi" => &[
        VerbDictionaryEntry { lemma: "slåditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "slěditi" => &[
        VerbDictionaryEntry { lemma: "slěditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "slěpnųti" => &[
        VerbDictionaryEntry { lemma: "slěpnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "slųčati" => &[
        VerbDictionaryEntry { lemma: "slųčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "slųčiti" => &[
        VerbDictionaryEntry { lemma: "slųčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "smatrjati" => &[
        VerbDictionaryEntry { lemma: "smatrjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "smažiti" => &[
        VerbDictionaryEntry { lemma: "smažiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "smlåtiti" => &[
        VerbDictionaryEntry { lemma: "smlåtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "smlěti" => &[
        VerbDictionaryEntry { lemma: "smlěti", addition: "(smelje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "smogti" => &[
        VerbDictionaryEntry { lemma: "smogti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "smoliti" => &[
        VerbDictionaryEntry { lemma: "smoliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "smrkati" => &[
        VerbDictionaryEntry { lemma: "smrkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "smrščiti" => &[
        VerbDictionaryEntry { lemma: "smrščiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "smućati" => &[
        VerbDictionaryEntry { lemma: "smućati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "smykati" => &[
        VerbDictionaryEntry { lemma: "smykati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "smėnšati" => &[
        VerbDictionaryEntry { lemma: "smėnšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "smėnšiti" => &[
        VerbDictionaryEntry { lemma: "smėnšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "smękčati" => &[
        VerbDictionaryEntry { lemma: "smękčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "smękčiti" => &[
        VerbDictionaryEntry { lemma: "smękčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "smęčkati" => &[
        VerbDictionaryEntry { lemma: "smęčkati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "směti" => &[
        VerbDictionaryEntry { lemma: "směti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "směšati" => &[
        VerbDictionaryEntry { lemma: "směšati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "směšivati" => &[
        VerbDictionaryEntry { lemma: "směšivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "smŕditi" => &[
        VerbDictionaryEntry { lemma: "smŕditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "smųtiti" => &[
        VerbDictionaryEntry { lemma: "smųtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "smųćati" => &[
        VerbDictionaryEntry { lemma: "smųćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "snabděti" => &[
        VerbDictionaryEntry { lemma: "snabděti", addition: "(snabdi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "snabděvati" => &[
        VerbDictionaryEntry { lemma: "snabděvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "snesti" => &[
        VerbDictionaryEntry { lemma: "snesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "snetvarjati" => &[
        VerbDictionaryEntry { lemma: "snetvarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "snetvoriti" => &[
        VerbDictionaryEntry { lemma: "snetvoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "snimati" => &[
        VerbDictionaryEntry { lemma: "snimati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "snimati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sniti" => &[
        VerbDictionaryEntry { lemma: "sniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sniziti" => &[
        VerbDictionaryEntry { lemma: "sniziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "snižati" => &[
        VerbDictionaryEntry { lemma: "snižati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "snositi" => &[
        VerbDictionaryEntry { lemma: "snositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "snovati" => &[
        VerbDictionaryEntry { lemma: "snovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "snědati" => &[
        VerbDictionaryEntry { lemma: "snědati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sněžiti" => &[
        VerbDictionaryEntry { lemma: "sněžiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "soliti" => &[
        VerbDictionaryEntry { lemma: "soliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sondovati" => &[
        VerbDictionaryEntry { lemma: "sondovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "sortovati" => &[
        VerbDictionaryEntry { lemma: "sortovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "spadati" => &[
        VerbDictionaryEntry { lemma: "spadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "spakovati" => &[
        VerbDictionaryEntry { lemma: "spakovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "spaliti" => &[
        VerbDictionaryEntry { lemma: "spaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "spasati" => &[
        VerbDictionaryEntry { lemma: "spasati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "spasti" => &[
        VerbDictionaryEntry { lemma: "spasti", addition: "(spade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "spasti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "spati" => &[
        VerbDictionaryEntry { lemma: "spati", addition: "(spi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "spekulovati" => &[
        VerbDictionaryEntry { lemma: "spekulovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "spisyvati" => &[
        VerbDictionaryEntry { lemma: "spisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "splesti" => &[
        VerbDictionaryEntry { lemma: "splesti", addition: "(splete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sploditi" => &[
        VerbDictionaryEntry { lemma: "sploditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "spolupracovati" => &[
        VerbDictionaryEntry { lemma: "spolupracovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "spoluråbotyvati" => &[
        VerbDictionaryEntry { lemma: "spoluråbotyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "spomaliti" => &[
        VerbDictionaryEntry { lemma: "spomaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "spomaljati" => &[
        VerbDictionaryEntry { lemma: "spomaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "spominati" => &[
        VerbDictionaryEntry { lemma: "spominati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "spomněti" => &[
        VerbDictionaryEntry { lemma: "spomněti", addition: "(spomni)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sporiti" => &[
        VerbDictionaryEntry { lemma: "sporiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "spotrěbiti" => &[
        VerbDictionaryEntry { lemma: "spotrěbiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "spotrěbovati" => &[
        VerbDictionaryEntry { lemma: "spotrěbovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "spotěti" => &[
        VerbDictionaryEntry { lemma: "spotěti", addition: "(spoti)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "spoznati" => &[
        VerbDictionaryEntry { lemma: "spoznati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "spoznavati" => &[
        VerbDictionaryEntry { lemma: "spoznavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sprašati" => &[
        VerbDictionaryEntry { lemma: "sprašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sprositi" => &[
        VerbDictionaryEntry { lemma: "sprositi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "spręgati" => &[
        VerbDictionaryEntry { lemma: "spręgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "spręsti" => &[
        VerbDictionaryEntry { lemma: "spręsti", addition: "(spręde)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "spuhnųti" => &[
        VerbDictionaryEntry { lemma: "spuhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "spustiti" => &[
        VerbDictionaryEntry { lemma: "spustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "spušćati" => &[
        VerbDictionaryEntry { lemma: "spušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "spytati" => &[
        VerbDictionaryEntry { lemma: "spytati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "spěvati" => &[
        VerbDictionaryEntry { lemma: "spěvati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "spěšiti" => &[
        VerbDictionaryEntry { lemma: "spěšiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "srati" => &[
        VerbDictionaryEntry { lemma: "srati", addition: "(sere)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sruinovati" => &[
        VerbDictionaryEntry { lemma: "sruinovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sråmiti" => &[
        VerbDictionaryEntry { lemma: "sråmiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sråvniti" => &[
        VerbDictionaryEntry { lemma: "sråvniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sråvnjati" => &[
        VerbDictionaryEntry { lemma: "sråvnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sråzuměti" => &[
        VerbDictionaryEntry { lemma: "sråzuměti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "srųbati" => &[
        VerbDictionaryEntry { lemma: "srųbati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "srųbyvati" => &[
        VerbDictionaryEntry { lemma: "srųbyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "stabilizovati" => &[
        VerbDictionaryEntry { lemma: "stabilizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "standardizovati" => &[
        VerbDictionaryEntry { lemma: "standardizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "stanoviti" => &[
        VerbDictionaryEntry { lemma: "stanoviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "stanųti" => &[
        VerbDictionaryEntry { lemma: "stanųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "stapjati" => &[
        VerbDictionaryEntry { lemma: "stapjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "startovati" => &[
        VerbDictionaryEntry { lemma: "startovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "starěti" => &[
        VerbDictionaryEntry { lemma: "starěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "stati" => &[
        VerbDictionaryEntry { lemma: "stati", addition: "(stane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "stavati" => &[
        VerbDictionaryEntry { lemma: "stavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "staviti" => &[
        VerbDictionaryEntry { lemma: "staviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "stavjati" => &[
        VerbDictionaryEntry { lemma: "stavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "stačiti" => &[
        VerbDictionaryEntry { lemma: "stačiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "stekti" => &[
        VerbDictionaryEntry { lemma: "stekti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "steliti" => &[
        VerbDictionaryEntry { lemma: "steliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sterilizovati" => &[
        VerbDictionaryEntry { lemma: "sterilizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "stigati" => &[
        VerbDictionaryEntry { lemma: "stigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "stignųti" => &[
        VerbDictionaryEntry { lemma: "stignųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "stimulovati" => &[
        VerbDictionaryEntry { lemma: "stimulovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "stiskati" => &[
        VerbDictionaryEntry { lemma: "stiskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "stisknųti" => &[
        VerbDictionaryEntry { lemma: "stisknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "stlati" => &[
        VerbDictionaryEntry { lemma: "stlati", addition: "(stelje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "stojati" => &[
        VerbDictionaryEntry { lemma: "stojati", addition: "(stoji)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "stojati", addition: "(stoji)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "stonati" => &[
        VerbDictionaryEntry { lemma: "stonati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "stopiti" => &[
        VerbDictionaryEntry { lemma: "stopiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "stradati" => &[
        VerbDictionaryEntry { lemma: "stradati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "strahovati" => &[
        VerbDictionaryEntry { lemma: "strahovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "strajkovati" => &[
        VerbDictionaryEntry { lemma: "strajkovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "strašiti" => &[
        VerbDictionaryEntry { lemma: "strašiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "strigti" => &[
        VerbDictionaryEntry { lemma: "strigti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "strimati" => &[
        VerbDictionaryEntry { lemma: "strimati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "strimyvati" => &[
        VerbDictionaryEntry { lemma: "strimyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "strojiti" => &[
        VerbDictionaryEntry { lemma: "strojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "strugati" => &[
        VerbDictionaryEntry { lemma: "strugati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "strčiti" => &[
        VerbDictionaryEntry { lemma: "strčiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "strěgti" => &[
        VerbDictionaryEntry { lemma: "strěgti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "strěliti" => &[
        VerbDictionaryEntry { lemma: "strěliti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "strěljati" => &[
        VerbDictionaryEntry { lemma: "strěljati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "strěsti" => &[
        VerbDictionaryEntry { lemma: "strěsti", addition: "(strěte)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "strěćati" => &[
        VerbDictionaryEntry { lemma: "strěćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "studiovati" => &[
        VerbDictionaryEntry { lemma: "studiovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "studiti" => &[
        VerbDictionaryEntry { lemma: "studiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "stukati" => &[
        VerbDictionaryEntry { lemma: "stukati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "stuknųti" => &[
        VerbDictionaryEntry { lemma: "stuknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "stvarjati" => &[
        VerbDictionaryEntry { lemma: "stvarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "stvoriti" => &[
        VerbDictionaryEntry { lemma: "stvoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "stųpati" => &[
        VerbDictionaryEntry { lemma: "stųpati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "stųpiti" => &[
        VerbDictionaryEntry { lemma: "stųpiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "sunųti" => &[
        VerbDictionaryEntry { lemma: "sunųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "surfovati" => &[
        VerbDictionaryEntry { lemma: "surfovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "suvati" => &[
        VerbDictionaryEntry { lemma: "suvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sušiti" => &[
        VerbDictionaryEntry { lemma: "sušiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "svabiti" => &[
        VerbDictionaryEntry { lemma: "svabiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "svariti" => &[
        VerbDictionaryEntry { lemma: "svariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "svarjati" => &[
        VerbDictionaryEntry { lemma: "svarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "svistati" => &[
        VerbDictionaryEntry { lemma: "svistati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "svistnųti" => &[
        VerbDictionaryEntry { lemma: "svistnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "sviti" => &[
        VerbDictionaryEntry { lemma: "sviti", addition: "(svije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "svętiti" => &[
        VerbDictionaryEntry { lemma: "svętiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "svęzati" => &[
        VerbDictionaryEntry { lemma: "svęzati", addition: "(svęže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "svęzyvati" => &[
        VerbDictionaryEntry { lemma: "svęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "svědčiti" => &[
        VerbDictionaryEntry { lemma: "svědčiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "světiti" => &[
        VerbDictionaryEntry { lemma: "světiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "svŕběti" => &[
        VerbDictionaryEntry { lemma: "svŕběti", addition: "(svŕbi)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "svŕgati" => &[
        VerbDictionaryEntry { lemma: "svŕgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "svŕgnųti" => &[
        VerbDictionaryEntry { lemma: "svŕgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sytiti" => &[
        VerbDictionaryEntry { lemma: "sytiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sčisliti" => &[
        VerbDictionaryEntry { lemma: "sčisliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sčitati" => &[
        VerbDictionaryEntry { lemma: "sčitati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sęgati" => &[
        VerbDictionaryEntry { lemma: "sęgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sęgnųti" => &[
        VerbDictionaryEntry { lemma: "sęgnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "sědati" => &[
        VerbDictionaryEntry { lemma: "sědati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sědnųti" => &[
        VerbDictionaryEntry { lemma: "sědnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "sěděti" => &[
        VerbDictionaryEntry { lemma: "sěděti", addition: "(sědi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sějati" => &[
        VerbDictionaryEntry { lemma: "sějati", addition: "(sěje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sěkati" => &[
        VerbDictionaryEntry { lemma: "sěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sěknųti" => &[
        VerbDictionaryEntry { lemma: "sěknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sěkti" => &[
        VerbDictionaryEntry { lemma: "sěkti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sěsti" => &[
        VerbDictionaryEntry { lemma: "sěsti", addition: "(sěde)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "sŕditi" => &[
        VerbDictionaryEntry { lemma: "sŕditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sųditi" => &[
        VerbDictionaryEntry { lemma: "sųditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sųharmonizovati" => &[
        VerbDictionaryEntry { lemma: "sųharmonizovati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "sųpostaviti" => &[
        VerbDictionaryEntry { lemma: "sųpostaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sųpostavjati" => &[
        VerbDictionaryEntry { lemma: "sųpostavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sųprovađati" => &[
        VerbDictionaryEntry { lemma: "sųprovađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sųprovoditi" => &[
        VerbDictionaryEntry { lemma: "sųprovoditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sųråbotati" => &[
        VerbDictionaryEntry { lemma: "sųråbotati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sųsrědotočati" => &[
        VerbDictionaryEntry { lemma: "sųsrědotočati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sųsrědotočiti" => &[
        VerbDictionaryEntry { lemma: "sųsrědotočiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sųstrěsti" => &[
        VerbDictionaryEntry { lemma: "sųstrěsti", addition: "(sųstrěte)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sųstrěćati" => &[
        VerbDictionaryEntry { lemma: "sųstrěćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sųt" => &[
        VerbDictionaryEntry { lemma: "sųt", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sųtruditi" => &[
        VerbDictionaryEntry { lemma: "sųtruditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sųtstvovati" => &[
        VerbDictionaryEntry { lemma: "sųtstvovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sȯbrati" => &[
        VerbDictionaryEntry { lemma: "sȯbrati", addition: "(sbere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sȯdŕžati" => &[
        VerbDictionaryEntry { lemma: "sȯdŕžati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sȯdŕživati" => &[
        VerbDictionaryEntry { lemma: "sȯdŕživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sȯhnųti" => &[
        VerbDictionaryEntry { lemma: "sȯhnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sȯjdti" => &[
        VerbDictionaryEntry { lemma: "sȯjdti", addition: "(sȯjde; sȯšėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "sȯmknųti" => &[
        VerbDictionaryEntry { lemma: "sȯmknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sȯobćati" => &[
        VerbDictionaryEntry { lemma: "sȯobćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sȯobćiti" => &[
        VerbDictionaryEntry { lemma: "sȯobćiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sȯsati" => &[
        VerbDictionaryEntry { lemma: "sȯsati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sȯstaviti" => &[
        VerbDictionaryEntry { lemma: "sȯstaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sȯstavjati" => &[
        VerbDictionaryEntry { lemma: "sȯstavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sȯtkati" => &[
        VerbDictionaryEntry { lemma: "sȯtkati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sȯvladnųti" => &[
        VerbDictionaryEntry { lemma: "sȯvladnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sȯvladyvati" => &[
        VerbDictionaryEntry { lemma: "sȯvladyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sȯvpadati" => &[
        VerbDictionaryEntry { lemma: "sȯvpadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sȯvětovati" => &[
        VerbDictionaryEntry { lemma: "sȯvětovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sȯvŕšati" => &[
        VerbDictionaryEntry { lemma: "sȯvŕšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "sȯvŕšati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sȯvŕšiti" => &[
        VerbDictionaryEntry { lemma: "sȯvŕšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "sȯvŕšiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "sȯzdati" => &[
        VerbDictionaryEntry { lemma: "sȯzdati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sȯzdavati" => &[
        VerbDictionaryEntry { lemma: "sȯzdavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sȯzvati" => &[
        VerbDictionaryEntry { lemma: "sȯzvati", addition: "(sȯzȯve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sȯzyvati" => &[
        VerbDictionaryEntry { lemma: "sȯzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "sȯčuvstvovati" => &[
        VerbDictionaryEntry { lemma: "sȯčuvstvovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "sȯžrti" => &[
        VerbDictionaryEntry { lemma: "sȯžrti", addition: "(sȯžre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "sȯžrěti" => &[
        VerbDictionaryEntry { lemma: "sȯžrěti", addition: "(sȯžre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "tajati" => &[
        VerbDictionaryEntry { lemma: "tajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "tajiti" => &[
        VerbDictionaryEntry { lemma: "tajiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "tancevati" => &[
        VerbDictionaryEntry { lemma: "tancevati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "tatuovati" => &[
        VerbDictionaryEntry { lemma: "tatuovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "tekti" => &[
        VerbDictionaryEntry { lemma: "tekti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "telefonovati" => &[
        VerbDictionaryEntry { lemma: "telefonovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "tesati" => &[
        VerbDictionaryEntry { lemma: "tesati", addition: "(teše)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "testovati" => &[
        VerbDictionaryEntry { lemma: "testovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "tipkati" => &[
        VerbDictionaryEntry { lemma: "tipkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "tiskati" => &[
        VerbDictionaryEntry { lemma: "tiskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "tisknųti" => &[
        VerbDictionaryEntry { lemma: "tisknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "tkati" => &[
        VerbDictionaryEntry { lemma: "tkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "tknųti" => &[
        VerbDictionaryEntry { lemma: "tknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "tlěti" => &[
        VerbDictionaryEntry { lemma: "tlěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "tonųti" => &[
        VerbDictionaryEntry { lemma: "tonųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "topiti" => &[
        VerbDictionaryEntry { lemma: "topiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "točiti" => &[
        VerbDictionaryEntry { lemma: "točiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "trajati" => &[
        VerbDictionaryEntry { lemma: "trajati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "transkribovati" => &[
        VerbDictionaryEntry { lemma: "transkribovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "transliterovati" => &[
        VerbDictionaryEntry { lemma: "transliterovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "transportovati" => &[
        VerbDictionaryEntry { lemma: "transportovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "tratiti" => &[
        VerbDictionaryEntry { lemma: "tratiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "travmatizovati" => &[
        VerbDictionaryEntry { lemma: "travmatizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "trenovati" => &[
        VerbDictionaryEntry { lemma: "trenovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "trepetati" => &[
        VerbDictionaryEntry { lemma: "trepetati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "trgati" => &[
        VerbDictionaryEntry { lemma: "trgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "trgnųti" => &[
        VerbDictionaryEntry { lemma: "trgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "trgovati" => &[
        VerbDictionaryEntry { lemma: "trgovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "trimati" => &[
        VerbDictionaryEntry { lemma: "trimati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "trvati" => &[
        VerbDictionaryEntry { lemma: "trvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "trėvožiti" => &[
        VerbDictionaryEntry { lemma: "trėvožiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "tręsti" => &[
        VerbDictionaryEntry { lemma: "tręsti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "trěbovati" => &[
        VerbDictionaryEntry { lemma: "trěbovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "trěti" => &[
        VerbDictionaryEntry { lemma: "trěti", addition: "(tre)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "trězvěti" => &[
        VerbDictionaryEntry { lemma: "trězvěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "trųbiti" => &[
        VerbDictionaryEntry { lemma: "trųbiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "tvoriti" => &[
        VerbDictionaryEntry { lemma: "tvoriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "tvŕditi" => &[
        VerbDictionaryEntry { lemma: "tvŕditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "tvŕdnųti" => &[
        VerbDictionaryEntry { lemma: "tvŕdnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "tvŕděti" => &[
        VerbDictionaryEntry { lemma: "tvŕděti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "tykati" => &[
        VerbDictionaryEntry { lemma: "tykati", addition: "(tyče)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "tykati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "tėmněti" => &[
        VerbDictionaryEntry { lemma: "tėmněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "tęgati" => &[
        VerbDictionaryEntry { lemma: "tęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "tęgnųti" => &[
        VerbDictionaryEntry { lemma: "tęgnųti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "tŕpěti" => &[
        VerbDictionaryEntry { lemma: "tŕpěti", addition: "(tŕpi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "tŕpěti", addition: "(tŕpi)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "tŕti" => &[
        VerbDictionaryEntry { lemma: "tŕti", addition: "(tre)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "tųpěti" => &[
        VerbDictionaryEntry { lemma: "tųpěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "tųžiti" => &[
        VerbDictionaryEntry { lemma: "tųžiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "tȯlkti" => &[
        VerbDictionaryEntry { lemma: "tȯlkti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "tȯlmačiti" => &[
        VerbDictionaryEntry { lemma: "tȯlmačiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ubiti" => &[
        VerbDictionaryEntry { lemma: "ubiti", addition: "(ubije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ubivati" => &[
        VerbDictionaryEntry { lemma: "ubivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uběditi" => &[
        VerbDictionaryEntry { lemma: "uběditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uběgati" => &[
        VerbDictionaryEntry { lemma: "uběgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "uběgti" => &[
        VerbDictionaryEntry { lemma: "uběgti", addition: "(uběži)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "uběđati" => &[
        VerbDictionaryEntry { lemma: "uběđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "udaliti" => &[
        VerbDictionaryEntry { lemma: "udaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "udaljati" => &[
        VerbDictionaryEntry { lemma: "udaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "udariti" => &[
        VerbDictionaryEntry { lemma: "udariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "udarjati" => &[
        VerbDictionaryEntry { lemma: "udarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "udaviti" => &[
        VerbDictionaryEntry { lemma: "udaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "udiviti" => &[
        VerbDictionaryEntry { lemma: "udiviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "udivjati" => &[
        VerbDictionaryEntry { lemma: "udivjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "udoskonaliti" => &[
        VerbDictionaryEntry { lemma: "udoskonaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "udušiti" => &[
        VerbDictionaryEntry { lemma: "udušiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "udvojiti" => &[
        VerbDictionaryEntry { lemma: "udvojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uděliti" => &[
        VerbDictionaryEntry { lemma: "uděliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uděljati" => &[
        VerbDictionaryEntry { lemma: "uděljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "udŕžati" => &[
        VerbDictionaryEntry { lemma: "udŕžati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "udŕživati" => &[
        VerbDictionaryEntry { lemma: "udŕživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ugadati" => &[
        VerbDictionaryEntry { lemma: "ugadati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ugadyvati" => &[
        VerbDictionaryEntry { lemma: "ugadyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ugasati" => &[
        VerbDictionaryEntry { lemma: "ugasati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ugasnųti" => &[
        VerbDictionaryEntry { lemma: "ugasnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "uględati" => &[
        VerbDictionaryEntry { lemma: "uględati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uględěti" => &[
        VerbDictionaryEntry { lemma: "uględěti", addition: "(uględi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ugryzti" => &[
        VerbDictionaryEntry { lemma: "ugryzti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uhoditi" => &[
        VerbDictionaryEntry { lemma: "uhoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ujdti" => &[
        VerbDictionaryEntry { lemma: "ujdti", addition: "(ujde; ušėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "ujediniti" => &[
        VerbDictionaryEntry { lemma: "ujediniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ujedinjati" => &[
        VerbDictionaryEntry { lemma: "ujedinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ujehati" => &[
        VerbDictionaryEntry { lemma: "ujehati", addition: "(ujede)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "uježđati" => &[
        VerbDictionaryEntry { lemma: "uježđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ujmati" => &[
        VerbDictionaryEntry { lemma: "ujmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ujęti" => &[
        VerbDictionaryEntry { lemma: "ujęti", addition: "(ujme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ukazati" => &[
        VerbDictionaryEntry { lemma: "ukazati", addition: "(ukaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ukazyvati" => &[
        VerbDictionaryEntry { lemma: "ukazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uklåti" => &[
        VerbDictionaryEntry { lemma: "uklåti", addition: "(ukolje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ukrasiti" => &[
        VerbDictionaryEntry { lemma: "ukrasiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ukrasti" => &[
        VerbDictionaryEntry { lemma: "ukrasti", addition: "(ukrade)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ukrašati" => &[
        VerbDictionaryEntry { lemma: "ukrašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ukrotiti" => &[
        VerbDictionaryEntry { lemma: "ukrotiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ukryti" => &[
        VerbDictionaryEntry { lemma: "ukryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ukryvati" => &[
        VerbDictionaryEntry { lemma: "ukryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ukrěpiti" => &[
        VerbDictionaryEntry { lemma: "ukrěpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ukrěpjati" => &[
        VerbDictionaryEntry { lemma: "ukrěpjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ukųsiti" => &[
        VerbDictionaryEntry { lemma: "ukųsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ulagađati" => &[
        VerbDictionaryEntry { lemma: "ulagađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ulagoditi" => &[
        VerbDictionaryEntry { lemma: "ulagoditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uletěti" => &[
        VerbDictionaryEntry { lemma: "uletěti", addition: "(uleti)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "ulučati" => &[
        VerbDictionaryEntry { lemma: "ulučati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ulučiti" => &[
        VerbDictionaryEntry { lemma: "ulučiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ulučšati" => &[
        VerbDictionaryEntry { lemma: "ulučšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ulučšiti" => &[
        VerbDictionaryEntry { lemma: "ulučšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ulėgšati" => &[
        VerbDictionaryEntry { lemma: "ulėgšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ulėgšiti" => &[
        VerbDictionaryEntry { lemma: "ulėgšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ulěpšati" => &[
        VerbDictionaryEntry { lemma: "ulěpšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ulěpšiti" => &[
        VerbDictionaryEntry { lemma: "ulěpšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ulětati" => &[
        VerbDictionaryEntry { lemma: "ulětati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "umarjati" => &[
        VerbDictionaryEntry { lemma: "umarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "umirati" => &[
        VerbDictionaryEntry { lemma: "umirati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "umoliti" => &[
        VerbDictionaryEntry { lemma: "umoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "umoljati" => &[
        VerbDictionaryEntry { lemma: "umoljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "umoriti" => &[
        VerbDictionaryEntry { lemma: "umoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "umožniti" => &[
        VerbDictionaryEntry { lemma: "umožniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "umožnjati" => &[
        VerbDictionaryEntry { lemma: "umožnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "umrěti" => &[
        VerbDictionaryEntry { lemma: "umrěti", addition: "(umre)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "umyti" => &[
        VerbDictionaryEntry { lemma: "umyti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "umyvati" => &[
        VerbDictionaryEntry { lemma: "umyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "umėnšati" => &[
        VerbDictionaryEntry { lemma: "umėnšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "umėnšiti" => &[
        VerbDictionaryEntry { lemma: "umėnšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uměriti" => &[
        VerbDictionaryEntry { lemma: "uměriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uměrjati" => &[
        VerbDictionaryEntry { lemma: "uměrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uměstiti" => &[
        VerbDictionaryEntry { lemma: "uměstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uměti" => &[
        VerbDictionaryEntry { lemma: "uměti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uměšćati" => &[
        VerbDictionaryEntry { lemma: "uměšćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "umŕti" => &[
        VerbDictionaryEntry { lemma: "umŕti", addition: "(umre)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "umŕtviti" => &[
        VerbDictionaryEntry { lemma: "umŕtviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "umŕtvjati" => &[
        VerbDictionaryEntry { lemma: "umŕtvjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "unarodniti" => &[
        VerbDictionaryEntry { lemma: "unarodniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "unarodnjati" => &[
        VerbDictionaryEntry { lemma: "unarodnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "unemožniti" => &[
        VerbDictionaryEntry { lemma: "unemožniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "unemožnjati" => &[
        VerbDictionaryEntry { lemma: "unemožnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uneviniti" => &[
        VerbDictionaryEntry { lemma: "uneviniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "unevinjati" => &[
        VerbDictionaryEntry { lemma: "unevinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uniziti" => &[
        VerbDictionaryEntry { lemma: "uniziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uniščiti" => &[
        VerbDictionaryEntry { lemma: "uniščiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "unižati" => &[
        VerbDictionaryEntry { lemma: "unižati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "upakovati" => &[
        VerbDictionaryEntry { lemma: "upakovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "upakovyvati" => &[
        VerbDictionaryEntry { lemma: "upakovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "upasti" => &[
        VerbDictionaryEntry { lemma: "upasti", addition: "(upade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "upekti" => &[
        VerbDictionaryEntry { lemma: "upekti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uperiti" => &[
        VerbDictionaryEntry { lemma: "uperiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "upodabnjati" => &[
        VerbDictionaryEntry { lemma: "upodabnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "upodobniti" => &[
        VerbDictionaryEntry { lemma: "upodobniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "upokarnjati" => &[
        VerbDictionaryEntry { lemma: "upokarnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "upokorniti" => &[
        VerbDictionaryEntry { lemma: "upokorniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "upotrěbiti" => &[
        VerbDictionaryEntry { lemma: "upotrěbiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "upotrěbjati" => &[
        VerbDictionaryEntry { lemma: "upotrěbjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "upraviti" => &[
        VerbDictionaryEntry { lemma: "upraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "upravjati" => &[
        VerbDictionaryEntry { lemma: "upravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uprašćati" => &[
        VerbDictionaryEntry { lemma: "uprašćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uprostiti" => &[
        VerbDictionaryEntry { lemma: "uprostiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "upȯlnomoćevati" => &[
        VerbDictionaryEntry { lemma: "upȯlnomoćevati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "upȯlnomoćiti" => &[
        VerbDictionaryEntry { lemma: "upȯlnomoćiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uraziti" => &[
        VerbDictionaryEntry { lemma: "uraziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uražati" => &[
        VerbDictionaryEntry { lemma: "uražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uroditi" => &[
        VerbDictionaryEntry { lemma: "uroditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "urvati" => &[
        VerbDictionaryEntry { lemma: "urvati", addition: "(urve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uryvati" => &[
        VerbDictionaryEntry { lemma: "uryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uråvniti" => &[
        VerbDictionaryEntry { lemma: "uråvniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uråvnjati" => &[
        VerbDictionaryEntry { lemma: "uråvnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uręditi" => &[
        VerbDictionaryEntry { lemma: "uręditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uręđati" => &[
        VerbDictionaryEntry { lemma: "uręđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "urěkati" => &[
        VerbDictionaryEntry { lemma: "urěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "urěkti" => &[
        VerbDictionaryEntry { lemma: "urěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "urězyvati" => &[
        VerbDictionaryEntry { lemma: "urězyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "usiliti" => &[
        VerbDictionaryEntry { lemma: "usiliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "usiljati" => &[
        VerbDictionaryEntry { lemma: "usiljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "usilovati" => &[
        VerbDictionaryEntry { lemma: "usilovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "uskoriti" => &[
        VerbDictionaryEntry { lemma: "uskoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uskorjati" => &[
        VerbDictionaryEntry { lemma: "uskorjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "usložniti" => &[
        VerbDictionaryEntry { lemma: "usložniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "usložnjati" => &[
        VerbDictionaryEntry { lemma: "usložnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uslyšati" => &[
        VerbDictionaryEntry { lemma: "uslyšati", addition: "(uslyši)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "usmŕtiti" => &[
        VerbDictionaryEntry { lemma: "usmŕtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "usmŕćati" => &[
        VerbDictionaryEntry { lemma: "usmŕćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "usnųti" => &[
        VerbDictionaryEntry { lemma: "usnųti", addition: "(usne)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "uspokajati" => &[
        VerbDictionaryEntry { lemma: "uspokajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uspokojiti" => &[
        VerbDictionaryEntry { lemma: "uspokojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "usposabjati" => &[
        VerbDictionaryEntry { lemma: "usposabjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "usposobiti" => &[
        VerbDictionaryEntry { lemma: "usposobiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uspravědliviti" => &[
        VerbDictionaryEntry { lemma: "uspravědliviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uspravědlivjati" => &[
        VerbDictionaryEntry { lemma: "uspravědlivjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uspěti" => &[
        VerbDictionaryEntry { lemma: "uspěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "uspěvati" => &[
        VerbDictionaryEntry { lemma: "uspěvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ustaliti" => &[
        VerbDictionaryEntry { lemma: "ustaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ustaljati" => &[
        VerbDictionaryEntry { lemma: "ustaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ustanavjati" => &[
        VerbDictionaryEntry { lemma: "ustanavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ustanoviti" => &[
        VerbDictionaryEntry { lemma: "ustanoviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ustati" => &[
        VerbDictionaryEntry { lemma: "ustati", addition: "(ustane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "ustavati" => &[
        VerbDictionaryEntry { lemma: "ustavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ustaviti" => &[
        VerbDictionaryEntry { lemma: "ustaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ustavjati" => &[
        VerbDictionaryEntry { lemma: "ustavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ustrajati" => &[
        VerbDictionaryEntry { lemma: "ustrajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ustrojiti" => &[
        VerbDictionaryEntry { lemma: "ustrojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ustųpati" => &[
        VerbDictionaryEntry { lemma: "ustųpati", addition: "(+3)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "ustųpati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ustųpiti" => &[
        VerbDictionaryEntry { lemma: "ustųpiti", addition: "(+3)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "ustųpiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "usyhati" => &[
        VerbDictionaryEntry { lemma: "usyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "usěsti" => &[
        VerbDictionaryEntry { lemma: "usěsti", addition: "(usěde)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "usȯhnųti" => &[
        VerbDictionaryEntry { lemma: "usȯhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "utekti" => &[
        VerbDictionaryEntry { lemma: "utekti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "utekųćiniti" => &[
        VerbDictionaryEntry { lemma: "utekųćiniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "utekųćinjati" => &[
        VerbDictionaryEntry { lemma: "utekųćinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "utirati" => &[
        VerbDictionaryEntry { lemma: "utirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "utišati" => &[
        VerbDictionaryEntry { lemma: "utišati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "utišiti" => &[
        VerbDictionaryEntry { lemma: "utišiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "utonųti" => &[
        VerbDictionaryEntry { lemma: "utonųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "utopiti" => &[
        VerbDictionaryEntry { lemma: "utopiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "utratiti" => &[
        VerbDictionaryEntry { lemma: "utratiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "utraćati" => &[
        VerbDictionaryEntry { lemma: "utraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "utrudniti" => &[
        VerbDictionaryEntry { lemma: "utrudniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "utrudnjati" => &[
        VerbDictionaryEntry { lemma: "utrudnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "utrěti" => &[
        VerbDictionaryEntry { lemma: "utrěti", addition: "(utre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "utvŕditi" => &[
        VerbDictionaryEntry { lemma: "utvŕditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "utvŕđati" => &[
        VerbDictionaryEntry { lemma: "utvŕđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "utęžati" => &[
        VerbDictionaryEntry { lemma: "utęžati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "utęžiti" => &[
        VerbDictionaryEntry { lemma: "utęžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "utěkati" => &[
        VerbDictionaryEntry { lemma: "utěkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "utělesniti" => &[
        VerbDictionaryEntry { lemma: "utělesniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "utělesnjati" => &[
        VerbDictionaryEntry { lemma: "utělesnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "utěšati" => &[
        VerbDictionaryEntry { lemma: "utěšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "utěšiti" => &[
        VerbDictionaryEntry { lemma: "utěšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "utŕpěti" => &[
        VerbDictionaryEntry { lemma: "utŕpěti", addition: "(utŕpi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "utŕti" => &[
        VerbDictionaryEntry { lemma: "utŕti", addition: "(utre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "utȯčniti" => &[
        VerbDictionaryEntry { lemma: "utȯčniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "utȯčnjati" => &[
        VerbDictionaryEntry { lemma: "utȯčnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uvadnjati" => &[
        VerbDictionaryEntry { lemma: "uvadnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uvaljnjati" => &[
        VerbDictionaryEntry { lemma: "uvaljnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uvažati" => &[
        VerbDictionaryEntry { lemma: "uvažati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uvažiti" => &[
        VerbDictionaryEntry { lemma: "uvažiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uveličati" => &[
        VerbDictionaryEntry { lemma: "uveličati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uveličiti" => &[
        VerbDictionaryEntry { lemma: "uveličiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uviděti" => &[
        VerbDictionaryEntry { lemma: "uviděti", addition: "(uvidi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uvodniti" => &[
        VerbDictionaryEntry { lemma: "uvodniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uvoljniti" => &[
        VerbDictionaryEntry { lemma: "uvoljniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uvędati" => &[
        VerbDictionaryEntry { lemma: "uvędati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "uvędnųti" => &[
        VerbDictionaryEntry { lemma: "uvędnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "uvęznųti" => &[
        VerbDictionaryEntry { lemma: "uvęznųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "uvědamjati" => &[
        VerbDictionaryEntry { lemma: "uvědamjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uvědomiti" => &[
        VerbDictionaryEntry { lemma: "uvědomiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uvěkověčiti" => &[
        VerbDictionaryEntry { lemma: "uvěkověčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uvěriti" => &[
        VerbDictionaryEntry { lemma: "uvěriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uvěrjati" => &[
        VerbDictionaryEntry { lemma: "uvěrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uzdravjati" => &[
        VerbDictionaryEntry { lemma: "uzdravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uzdråviti" => &[
        VerbDictionaryEntry { lemma: "uzdråviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uznati" => &[
        VerbDictionaryEntry { lemma: "uznati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uznavati" => &[
        VerbDictionaryEntry { lemma: "uznavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "uzrěti" => &[
        VerbDictionaryEntry { lemma: "uzrěti", addition: "(uzri)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uzurpovati" => &[
        VerbDictionaryEntry { lemma: "uzurpovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "učarovati" => &[
        VerbDictionaryEntry { lemma: "učarovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "učarovyvati" => &[
        VerbDictionaryEntry { lemma: "učarovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "učiti" => &[
        VerbDictionaryEntry { lemma: "učiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "učęstvovati" => &[
        VerbDictionaryEntry { lemma: "učęstvovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "užasati" => &[
        VerbDictionaryEntry { lemma: "užasati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "užasnųti" => &[
        VerbDictionaryEntry { lemma: "užasnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "užiti" => &[
        VerbDictionaryEntry { lemma: "užiti", addition: "(užive)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "uživati" => &[
        VerbDictionaryEntry { lemma: "uživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vabiti" => &[
        VerbDictionaryEntry { lemma: "vabiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vajati" => &[
        VerbDictionaryEntry { lemma: "vajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "valiti" => &[
        VerbDictionaryEntry { lemma: "valiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "valjati" => &[
        VerbDictionaryEntry { lemma: "valjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "valjcevati" => &[
        VerbDictionaryEntry { lemma: "valjcevati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true },
    ],
    "valsovati" => &[
        VerbDictionaryEntry { lemma: "valsovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "variovati" => &[
        VerbDictionaryEntry { lemma: "variovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true },
    ],
    "variti" => &[
        VerbDictionaryEntry { lemma: "variti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "važiti" => &[
        VerbDictionaryEntry { lemma: "važiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vdyhati" => &[
        VerbDictionaryEntry { lemma: "vdyhati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vdȯhnųti" => &[
        VerbDictionaryEntry { lemma: "vdȯhnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "velěti" => &[
        VerbDictionaryEntry { lemma: "velěti", addition: "(veli)", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "ventilovati" => &[
        VerbDictionaryEntry { lemma: "ventilovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "verbovati" => &[
        VerbDictionaryEntry { lemma: "verbovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "verifikovati" => &[
        VerbDictionaryEntry { lemma: "verifikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "veseliti" => &[
        VerbDictionaryEntry { lemma: "veseliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vesti" => &[
        VerbDictionaryEntry { lemma: "vesti", addition: "(vede)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vezti" => &[
        VerbDictionaryEntry { lemma: "vezti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "večerjati" => &[
        VerbDictionaryEntry { lemma: "večerjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vhoditi" => &[
        VerbDictionaryEntry { lemma: "vhoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vibrovati" => &[
        VerbDictionaryEntry { lemma: "vibrovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true },
    ],
    "viděti" => &[
        VerbDictionaryEntry { lemma: "viděti", addition: "(vidi)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "visěti" => &[
        VerbDictionaryEntry { lemma: "visěti", addition: "(visi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vitati" => &[
        VerbDictionaryEntry { lemma: "vitati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "viti" => &[
        VerbDictionaryEntry { lemma: "viti", addition: "(vije)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vjehati" => &[
        VerbDictionaryEntry { lemma: "vjehati", addition: "(vjede)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "vježđati" => &[
        VerbDictionaryEntry { lemma: "vježđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vkladati" => &[
        VerbDictionaryEntry { lemma: "vkladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vključati" => &[
        VerbDictionaryEntry { lemma: "vključati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vključiti" => &[
        VerbDictionaryEntry { lemma: "vključiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vkųsiti" => &[
        VerbDictionaryEntry { lemma: "vkųsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vkųšati" => &[
        VerbDictionaryEntry { lemma: "vkųšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vladati" => &[
        VerbDictionaryEntry { lemma: "vladati", addition: "(+5)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vlagati" => &[
        VerbDictionaryEntry { lemma: "vlagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vlastniti" => &[
        VerbDictionaryEntry { lemma: "vlastniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vlivati" => &[
        VerbDictionaryEntry { lemma: "vlivati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vložiti" => &[
        VerbDictionaryEntry { lemma: "vložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vlåděti" => &[
        VerbDictionaryEntry { lemma: "vlåděti", addition: "(+5)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vlåčiti" => &[
        VerbDictionaryEntry { lemma: "vlåčiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vlåžiti" => &[
        VerbDictionaryEntry { lemma: "vlåžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vlěkti" => &[
        VerbDictionaryEntry { lemma: "vlěkti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vlězati" => &[
        VerbDictionaryEntry { lemma: "vlězati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vměstiti" => &[
        VerbDictionaryEntry { lemma: "vměstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vměšati" => &[
        VerbDictionaryEntry { lemma: "vměšati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vměšivati" => &[
        VerbDictionaryEntry { lemma: "vměšivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vměšćati" => &[
        VerbDictionaryEntry { lemma: "vměšćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vnesti" => &[
        VerbDictionaryEntry { lemma: "vnesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vnikati" => &[
        VerbDictionaryEntry { lemma: "vnikati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vniknųti" => &[
        VerbDictionaryEntry { lemma: "vniknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "vnositi" => &[
        VerbDictionaryEntry { lemma: "vnositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "voditi" => &[
        VerbDictionaryEntry { lemma: "voditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vojevati" => &[
        VerbDictionaryEntry { lemma: "vojevati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "volěti" => &[
        VerbDictionaryEntry { lemma: "volěti", addition: "(voli)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vonjati" => &[
        VerbDictionaryEntry { lemma: "vonjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "voziti" => &[
        VerbDictionaryEntry { lemma: "voziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "voščiti" => &[
        VerbDictionaryEntry { lemma: "voščiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vpadati" => &[
        VerbDictionaryEntry { lemma: "vpadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vpasti" => &[
        VerbDictionaryEntry { lemma: "vpasti", addition: "(vpade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "vpihati" => &[
        VerbDictionaryEntry { lemma: "vpihati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vpisati" => &[
        VerbDictionaryEntry { lemma: "vpisati", addition: "(vpiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vpisyvati" => &[
        VerbDictionaryEntry { lemma: "vpisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vpiti" => &[
        VerbDictionaryEntry { lemma: "vpiti", addition: "(vpije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vpivati" => &[
        VerbDictionaryEntry { lemma: "vpivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vplesti" => &[
        VerbDictionaryEntry { lemma: "vplesti", addition: "(vplete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vpletati" => &[
        VerbDictionaryEntry { lemma: "vpletati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vplyvati" => &[
        VerbDictionaryEntry { lemma: "vplyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vplyvti" => &[
        VerbDictionaryEntry { lemma: "vplyvti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "vpręgati" => &[
        VerbDictionaryEntry { lemma: "vpręgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vpręgti" => &[
        VerbDictionaryEntry { lemma: "vpręgti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vraćati" => &[
        VerbDictionaryEntry { lemma: "vraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vråtiti" => &[
        VerbDictionaryEntry { lemma: "vråtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vråžiti" => &[
        VerbDictionaryEntry { lemma: "vråžiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vrčati" => &[
        VerbDictionaryEntry { lemma: "vrčati", addition: "(vrči)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vrěti" => &[
        VerbDictionaryEntry { lemma: "vrěti", addition: "(vri)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vrěščati" => &[
        VerbDictionaryEntry { lemma: "vrěščati", addition: "(vrěšči)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vrųbati" => &[
        VerbDictionaryEntry { lemma: "vrųbati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vrųbyvati" => &[
        VerbDictionaryEntry { lemma: "vrųbyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vrųčati" => &[
        VerbDictionaryEntry { lemma: "vrųčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vrųčiti" => &[
        VerbDictionaryEntry { lemma: "vrųčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vsaditi" => &[
        VerbDictionaryEntry { lemma: "vsaditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vsađati" => &[
        VerbDictionaryEntry { lemma: "vsađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vskočiti" => &[
        VerbDictionaryEntry { lemma: "vskočiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "vstati" => &[
        VerbDictionaryEntry { lemma: "vstati", addition: "(vstane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "vstavati" => &[
        VerbDictionaryEntry { lemma: "vstavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vstaviti" => &[
        VerbDictionaryEntry { lemma: "vstaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vstavjati" => &[
        VerbDictionaryEntry { lemma: "vstavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vstųpati" => &[
        VerbDictionaryEntry { lemma: "vstųpati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vstųpiti" => &[
        VerbDictionaryEntry { lemma: "vstųpiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "vtiskati" => &[
        VerbDictionaryEntry { lemma: "vtiskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vtisknųti" => &[
        VerbDictionaryEntry { lemma: "vtisknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vtrgati" => &[
        VerbDictionaryEntry { lemma: "vtrgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vtrgnųti" => &[
        VerbDictionaryEntry { lemma: "vtrgnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "vtęgati" => &[
        VerbDictionaryEntry { lemma: "vtęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vtęgnųti" => &[
        VerbDictionaryEntry { lemma: "vtęgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vulkanizovati" => &[
        VerbDictionaryEntry { lemma: "vulkanizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "vvesti" => &[
        VerbDictionaryEntry { lemma: "vvesti", addition: "(vvede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vvezti" => &[
        VerbDictionaryEntry { lemma: "vvezti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vvoditi" => &[
        VerbDictionaryEntry { lemma: "vvoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vvoziti" => &[
        VerbDictionaryEntry { lemma: "vvoziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vvŕgati" => &[
        VerbDictionaryEntry { lemma: "vvŕgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vvŕgnųti" => &[
        VerbDictionaryEntry { lemma: "vvŕgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vyjdti" => &[
        VerbDictionaryEntry { lemma: "vyjdti", addition: "(vyjde; vyšėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "vyjmati" => &[
        VerbDictionaryEntry { lemma: "vyjmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vyryti" => &[
        VerbDictionaryEntry { lemma: "vyryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vyti" => &[
        VerbDictionaryEntry { lemma: "vyti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vzajėmodějati" => &[
        VerbDictionaryEntry { lemma: "vzajėmodějati", addition: "(vzajemoděje)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vzajėmodějstvovati" => &[
        VerbDictionaryEntry { lemma: "vzajėmodějstvovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vzęti" => &[
        VerbDictionaryEntry { lemma: "vzęti", addition: "(vȯzme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "včleniti" => &[
        VerbDictionaryEntry { lemma: "včleniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "včlenjati" => &[
        VerbDictionaryEntry { lemma: "včlenjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vędnųti" => &[
        VerbDictionaryEntry { lemma: "vędnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vęzati" => &[
        VerbDictionaryEntry { lemma: "vęzati", addition: "(vęže)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vęznųti" => &[
        VerbDictionaryEntry { lemma: "vęznųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "věděti" => &[
        VerbDictionaryEntry { lemma: "věděti", addition: "(vě)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vějati" => &[
        VerbDictionaryEntry { lemma: "vějati", addition: "(věje)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "věriti" => &[
        VerbDictionaryEntry { lemma: "věriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "větriti" => &[
        VerbDictionaryEntry { lemma: "větriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "věšati" => &[
        VerbDictionaryEntry { lemma: "věšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vŕgati" => &[
        VerbDictionaryEntry { lemma: "vŕgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vŕgnųti" => &[
        VerbDictionaryEntry { lemma: "vŕgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vŕnųti" => &[
        VerbDictionaryEntry { lemma: "vŕnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vŕtěti" => &[
        VerbDictionaryEntry { lemma: "vŕtěti", addition: "(vŕti)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
        VerbDictionaryEntry { lemma: "vŕtěti", addition: "(vŕti)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vȯjdti" => &[
        VerbDictionaryEntry { lemma: "vȯjdti", addition: "(vȯjde; všėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "vȯlgnųti" => &[
        VerbDictionaryEntry { lemma: "vȯlgnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vȯorųžiti" => &[
        VerbDictionaryEntry { lemma: "vȯorųžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯphati" => &[
        VerbDictionaryEntry { lemma: "vȯphati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯplȯtiti" => &[
        VerbDictionaryEntry { lemma: "vȯplȯtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯplȯćati" => &[
        VerbDictionaryEntry { lemma: "vȯplȯćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯzbogatiti" => &[
        VerbDictionaryEntry { lemma: "vȯzbogatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯzbogaćati" => &[
        VerbDictionaryEntry { lemma: "vȯzbogaćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯzbranjati" => &[
        VerbDictionaryEntry { lemma: "vȯzbranjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯzbråniti" => &[
        VerbDictionaryEntry { lemma: "vȯzbråniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯzbuditi" => &[
        VerbDictionaryEntry { lemma: "vȯzbuditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯzbuđati" => &[
        VerbDictionaryEntry { lemma: "vȯzbuđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯzdvignųti" => &[
        VerbDictionaryEntry { lemma: "vȯzdvignųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯzdyhati" => &[
        VerbDictionaryEntry { lemma: "vȯzdyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vȯzdȯhnųti" => &[
        VerbDictionaryEntry { lemma: "vȯzdȯhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "vȯzhoditi" => &[
        VerbDictionaryEntry { lemma: "vȯzhoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vȯzimati" => &[
        VerbDictionaryEntry { lemma: "vȯzimati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯzjęti" => &[
        VerbDictionaryEntry { lemma: "vȯzjęti", addition: "(vȯzȯjme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯzklicati" => &[
        VerbDictionaryEntry { lemma: "vȯzklicati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯzkliknųti" => &[
        VerbDictionaryEntry { lemma: "vȯzkliknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯzkresati" => &[
        VerbDictionaryEntry { lemma: "vȯzkresati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vȯzkresiti" => &[
        VerbDictionaryEntry { lemma: "vȯzkresiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯzkresnųti" => &[
        VerbDictionaryEntry { lemma: "vȯzkresnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "vȯzkrešati" => &[
        VerbDictionaryEntry { lemma: "vȯzkrešati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯzletěti" => &[
        VerbDictionaryEntry { lemma: "vȯzletěti", addition: "(vȯzleti)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "vȯzlětati" => &[
        VerbDictionaryEntry { lemma: "vȯzlětati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vȯznesti" => &[
        VerbDictionaryEntry { lemma: "vȯznesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯznikati" => &[
        VerbDictionaryEntry { lemma: "vȯznikati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vȯzniknųti" => &[
        VerbDictionaryEntry { lemma: "vȯzniknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "vȯznositi" => &[
        VerbDictionaryEntry { lemma: "vȯznositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯzpamętati" => &[
        VerbDictionaryEntry { lemma: "vȯzpamętati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯzpitati" => &[
        VerbDictionaryEntry { lemma: "vȯzpitati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯzpityvati" => &[
        VerbDictionaryEntry { lemma: "vȯzpityvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯzpominati" => &[
        VerbDictionaryEntry { lemma: "vȯzpominati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯzpomněti" => &[
        VerbDictionaryEntry { lemma: "vȯzpomněti", addition: "(vȯzpomni)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯzrastati" => &[
        VerbDictionaryEntry { lemma: "vȯzrastati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vȯzroditi" => &[
        VerbDictionaryEntry { lemma: "vȯzroditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯzråsti" => &[
        VerbDictionaryEntry { lemma: "vȯzråsti", addition: "(vȯzråste)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "vȯzsiliti" => &[
        VerbDictionaryEntry { lemma: "vȯzsiliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯzsilovati" => &[
        VerbDictionaryEntry { lemma: "vȯzsilovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯzskakati" => &[
        VerbDictionaryEntry { lemma: "vȯzskakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vȯzskočiti" => &[
        VerbDictionaryEntry { lemma: "vȯzskočiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "vȯzstati" => &[
        VerbDictionaryEntry { lemma: "vȯzstati", addition: "(vȯzstane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "vȯzstavati" => &[
        VerbDictionaryEntry { lemma: "vȯzstavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "vȯztręsati" => &[
        VerbDictionaryEntry { lemma: "vȯztręsati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯztręsti" => &[
        VerbDictionaryEntry { lemma: "vȯztręsti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯzveličati" => &[
        VerbDictionaryEntry { lemma: "vȯzveličati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯzveličivati" => &[
        VerbDictionaryEntry { lemma: "vȯzveličivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯzvesti" => &[
        VerbDictionaryEntry { lemma: "vȯzvesti", addition: "(vȯzvede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯzvoditi" => &[
        VerbDictionaryEntry { lemma: "vȯzvoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯzvraćati" => &[
        VerbDictionaryEntry { lemma: "vȯzvraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯzvråtiti" => &[
        VerbDictionaryEntry { lemma: "vȯzvråtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯzvysiti" => &[
        VerbDictionaryEntry { lemma: "vȯzvysiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯzvyšati" => &[
        VerbDictionaryEntry { lemma: "vȯzvyšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯzvŕšati" => &[
        VerbDictionaryEntry { lemma: "vȯzvŕšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "vȯzvŕšiti" => &[
        VerbDictionaryEntry { lemma: "vȯzvŕšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "vȯzȯjdti" => &[
        VerbDictionaryEntry { lemma: "vȯzȯjdti", addition: "(vȯzȯjde; vȯzšėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zabarikadovati" => &[
        VerbDictionaryEntry { lemma: "zabarikadovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zabaviti" => &[
        VerbDictionaryEntry { lemma: "zabaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zabavjati" => &[
        VerbDictionaryEntry { lemma: "zabavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zabezpamętiti" => &[
        VerbDictionaryEntry { lemma: "zabezpamętiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zabezpamęćati" => &[
        VerbDictionaryEntry { lemma: "zabezpamęćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zabezpečati" => &[
        VerbDictionaryEntry { lemma: "zabezpečati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zabezpečiti" => &[
        VerbDictionaryEntry { lemma: "zabezpečiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zabirati" => &[
        VerbDictionaryEntry { lemma: "zabirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zablokovati" => &[
        VerbDictionaryEntry { lemma: "zablokovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zablųditi" => &[
        VerbDictionaryEntry { lemma: "zablųditi", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zablųđati" => &[
        VerbDictionaryEntry { lemma: "zablųđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zabolěti" => &[
        VerbDictionaryEntry { lemma: "zabolěti", addition: "(zabolěje)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "zabolěti", addition: "(zaboli)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zabolěvati" => &[
        VerbDictionaryEntry { lemma: "zabolěvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zabranjati" => &[
        VerbDictionaryEntry { lemma: "zabranjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zabrati" => &[
        VerbDictionaryEntry { lemma: "zabrati", addition: "(zabere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zabråniti" => &[
        VerbDictionaryEntry { lemma: "zabråniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zabyti" => &[
        VerbDictionaryEntry { lemma: "zabyti", addition: "(zabųde)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zabyvati" => &[
        VerbDictionaryEntry { lemma: "zabyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zadati" => &[
        VerbDictionaryEntry { lemma: "zadati", addition: "(zada)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zadavati" => &[
        VerbDictionaryEntry { lemma: "zadavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zadovaljati" => &[
        VerbDictionaryEntry { lemma: "zadovaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zadovoliti" => &[
        VerbDictionaryEntry { lemma: "zadovoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zadrěmati" => &[
        VerbDictionaryEntry { lemma: "zadrěmati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zadržati" => &[
        VerbDictionaryEntry { lemma: "zadržati", addition: "(zadŕži)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zadŕžati" => &[
        VerbDictionaryEntry { lemma: "zadŕžati", addition: "(zadŕži)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zadŕživati" => &[
        VerbDictionaryEntry { lemma: "zadŕživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zagladiti" => &[
        VerbDictionaryEntry { lemma: "zagladiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaglađati" => &[
        VerbDictionaryEntry { lemma: "zaglađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zagorěti" => &[
        VerbDictionaryEntry { lemma: "zagorěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zagospodariti" => &[
        VerbDictionaryEntry { lemma: "zagospodariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zagroziti" => &[
        VerbDictionaryEntry { lemma: "zagroziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zahoditi" => &[
        VerbDictionaryEntry { lemma: "zahoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zahvatiti" => &[
        VerbDictionaryEntry { lemma: "zahvatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zahvaćati" => &[
        VerbDictionaryEntry { lemma: "zahvaćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zahvorěti" => &[
        VerbDictionaryEntry { lemma: "zahvorěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zahvorěvati" => &[
        VerbDictionaryEntry { lemma: "zahvorěvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zainteresovati" => &[
        VerbDictionaryEntry { lemma: "zainteresovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zajdti" => &[
        VerbDictionaryEntry { lemma: "zajdti", addition: "(zajde; zašėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zajmati" => &[
        VerbDictionaryEntry { lemma: "zajmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zajęti" => &[
        VerbDictionaryEntry { lemma: "zajęti", addition: "(zajme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zakazyvati" => &[
        VerbDictionaryEntry { lemma: "zakazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zakašljati" => &[
        VerbDictionaryEntry { lemma: "zakašljati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zakladati" => &[
        VerbDictionaryEntry { lemma: "zakladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zaklinati" => &[
        VerbDictionaryEntry { lemma: "zaklinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zakliniti" => &[
        VerbDictionaryEntry { lemma: "zakliniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaklinovati" => &[
        VerbDictionaryEntry { lemma: "zaklinovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zaključati" => &[
        VerbDictionaryEntry { lemma: "zaključati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "zaključati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zaključiti" => &[
        VerbDictionaryEntry { lemma: "zaključiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "zaključiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaklåti" => &[
        VerbDictionaryEntry { lemma: "zaklåti", addition: "(zakolje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaklęti" => &[
        VerbDictionaryEntry { lemma: "zaklęti", addition: "(zaklne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zakončiti" => &[
        VerbDictionaryEntry { lemma: "zakončiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zakopati" => &[
        VerbDictionaryEntry { lemma: "zakopati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zakopyvati" => &[
        VerbDictionaryEntry { lemma: "zakopyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zakričati" => &[
        VerbDictionaryEntry { lemma: "zakričati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zakryti" => &[
        VerbDictionaryEntry { lemma: "zakryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zakryvati" => &[
        VerbDictionaryEntry { lemma: "zakryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zalajati" => &[
        VerbDictionaryEntry { lemma: "zalajati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "založiti" => &[
        VerbDictionaryEntry { lemma: "založiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zalězati" => &[
        VerbDictionaryEntry { lemma: "zalězati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zalězti" => &[
        VerbDictionaryEntry { lemma: "zalězti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zamesti" => &[
        VerbDictionaryEntry { lemma: "zamesti", addition: "(zamete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zamglati" => &[
        VerbDictionaryEntry { lemma: "zamglati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zamirati" => &[
        VerbDictionaryEntry { lemma: "zamirati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zamknųti" => &[
        VerbDictionaryEntry { lemma: "zamknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zamražati" => &[
        VerbDictionaryEntry { lemma: "zamražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zamrzati" => &[
        VerbDictionaryEntry { lemma: "zamrzati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zamrznųti" => &[
        VerbDictionaryEntry { lemma: "zamrznųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zamråziti" => &[
        VerbDictionaryEntry { lemma: "zamråziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zamrěti" => &[
        VerbDictionaryEntry { lemma: "zamrěti", addition: "(zamre)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zamykati" => &[
        VerbDictionaryEntry { lemma: "zamykati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zamysliti" => &[
        VerbDictionaryEntry { lemma: "zamysliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zamysljati" => &[
        VerbDictionaryEntry { lemma: "zamysljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zamėdliti" => &[
        VerbDictionaryEntry { lemma: "zamėdliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zamėdljati" => &[
        VerbDictionaryEntry { lemma: "zamėdljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zaměniti" => &[
        VerbDictionaryEntry { lemma: "zaměniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaměnjati" => &[
        VerbDictionaryEntry { lemma: "zaměnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zaměsiti" => &[
        VerbDictionaryEntry { lemma: "zaměsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zamětati" => &[
        VerbDictionaryEntry { lemma: "zamětati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zaměšati" => &[
        VerbDictionaryEntry { lemma: "zaměšati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaměšivati" => &[
        VerbDictionaryEntry { lemma: "zaměšivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zamŕti" => &[
        VerbDictionaryEntry { lemma: "zamŕti", addition: "(zamre)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zamȯlknųti" => &[
        VerbDictionaryEntry { lemma: "zamȯlknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zamȯlviti" => &[
        VerbDictionaryEntry { lemma: "zamȯlviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zamȯlvjati" => &[
        VerbDictionaryEntry { lemma: "zamȯlvjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zanedbati" => &[
        VerbDictionaryEntry { lemma: "zanedbati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zanedbyvati" => &[
        VerbDictionaryEntry { lemma: "zanedbyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zanepokojiti" => &[
        VerbDictionaryEntry { lemma: "zanepokojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zanuriti" => &[
        VerbDictionaryEntry { lemma: "zanuriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zanurjati" => &[
        VerbDictionaryEntry { lemma: "zanurjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zapadati" => &[
        VerbDictionaryEntry { lemma: "zapadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zapakovati" => &[
        VerbDictionaryEntry { lemma: "zapakovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zapakovyvati" => &[
        VerbDictionaryEntry { lemma: "zapakovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zapaliti" => &[
        VerbDictionaryEntry { lemma: "zapaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zapaljati" => &[
        VerbDictionaryEntry { lemma: "zapaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zapamętati" => &[
        VerbDictionaryEntry { lemma: "zapamętati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zapamętyvati" => &[
        VerbDictionaryEntry { lemma: "zapamętyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zapasti" => &[
        VerbDictionaryEntry { lemma: "zapasti", addition: "(zapade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zapečatati" => &[
        VerbDictionaryEntry { lemma: "zapečatati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zapečatyvati" => &[
        VerbDictionaryEntry { lemma: "zapečatyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zapisati" => &[
        VerbDictionaryEntry { lemma: "zapisati", addition: "(zapiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zapisyvati" => &[
        VerbDictionaryEntry { lemma: "zapisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zaplakati" => &[
        VerbDictionaryEntry { lemma: "zaplakati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zaplanovati" => &[
        VerbDictionaryEntry { lemma: "zaplanovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaplatiti" => &[
        VerbDictionaryEntry { lemma: "zaplatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaplěniti" => &[
        VerbDictionaryEntry { lemma: "zaplěniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaplěnjati" => &[
        VerbDictionaryEntry { lemma: "zaplěnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zapoznati" => &[
        VerbDictionaryEntry { lemma: "zapoznati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zapoznavati" => &[
        VerbDictionaryEntry { lemma: "zapoznavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "započinati" => &[
        VerbDictionaryEntry { lemma: "započinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "započęti" => &[
        VerbDictionaryEntry { lemma: "započęti", addition: "(započne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zapraviti" => &[
        VerbDictionaryEntry { lemma: "zapraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zapravjati" => &[
        VerbDictionaryEntry { lemma: "zapravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zaprojektovati" => &[
        VerbDictionaryEntry { lemma: "zaprojektovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zapråšiti" => &[
        VerbDictionaryEntry { lemma: "zapråšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zapręgati" => &[
        VerbDictionaryEntry { lemma: "zapręgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zapręgti" => &[
        VerbDictionaryEntry { lemma: "zapręgti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaprěčiti" => &[
        VerbDictionaryEntry { lemma: "zaprěčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zapustiti" => &[
        VerbDictionaryEntry { lemma: "zapustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zapušćati" => &[
        VerbDictionaryEntry { lemma: "zapušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zapytati" => &[
        VerbDictionaryEntry { lemma: "zapytati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zapytyvati" => &[
        VerbDictionaryEntry { lemma: "zapytyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zapȯlniti" => &[
        VerbDictionaryEntry { lemma: "zapȯlniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zapȯlnjati" => &[
        VerbDictionaryEntry { lemma: "zapȯlnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zarastati" => &[
        VerbDictionaryEntry { lemma: "zarastati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zaraziti" => &[
        VerbDictionaryEntry { lemma: "zaraziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaražati" => &[
        VerbDictionaryEntry { lemma: "zaražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zaregistrovati" => &[
        VerbDictionaryEntry { lemma: "zaregistrovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zarevti" => &[
        VerbDictionaryEntry { lemma: "zarevti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zarezervovati" => &[
        VerbDictionaryEntry { lemma: "zarezervovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zarydati" => &[
        VerbDictionaryEntry { lemma: "zarydati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zaråbotati" => &[
        VerbDictionaryEntry { lemma: "zaråbotati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaråbotyvati" => &[
        VerbDictionaryEntry { lemma: "zaråbotyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zaråsti" => &[
        VerbDictionaryEntry { lemma: "zaråsti", addition: "(zaråste)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zarųčati" => &[
        VerbDictionaryEntry { lemma: "zarųčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zarųčiti" => &[
        VerbDictionaryEntry { lemma: "zarųčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zasaditi" => &[
        VerbDictionaryEntry { lemma: "zasaditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zasađati" => &[
        VerbDictionaryEntry { lemma: "zasađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zaslanjati" => &[
        VerbDictionaryEntry { lemma: "zaslanjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zasloniti" => &[
        VerbDictionaryEntry { lemma: "zasloniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaslužiti" => &[
        VerbDictionaryEntry { lemma: "zaslužiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zasluživati" => &[
        VerbDictionaryEntry { lemma: "zasluživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zasnųti" => &[
        VerbDictionaryEntry { lemma: "zasnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zaspati" => &[
        VerbDictionaryEntry { lemma: "zaspati", addition: "(zaspi)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zasramjati" => &[
        VerbDictionaryEntry { lemma: "zasramjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zasrati" => &[
        VerbDictionaryEntry { lemma: "zasrati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zasråmiti" => &[
        VerbDictionaryEntry { lemma: "zasråmiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zastariti" => &[
        VerbDictionaryEntry { lemma: "zastariti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zastarjati" => &[
        VerbDictionaryEntry { lemma: "zastarjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zastati" => &[
        VerbDictionaryEntry { lemma: "zastati", addition: "(zastane)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zastavati" => &[
        VerbDictionaryEntry { lemma: "zastavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zastaviti" => &[
        VerbDictionaryEntry { lemma: "zastaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zastavjati" => &[
        VerbDictionaryEntry { lemma: "zastavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zastrašati" => &[
        VerbDictionaryEntry { lemma: "zastrašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zastrašiti" => &[
        VerbDictionaryEntry { lemma: "zastrašiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zastrěliti" => &[
        VerbDictionaryEntry { lemma: "zastrěliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zastrěljati" => &[
        VerbDictionaryEntry { lemma: "zastrěljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zastųpati" => &[
        VerbDictionaryEntry { lemma: "zastųpati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zastųpiti" => &[
        VerbDictionaryEntry { lemma: "zastųpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zasvěćati" => &[
        VerbDictionaryEntry { lemma: "zasvěćati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zasějati" => &[
        VerbDictionaryEntry { lemma: "zasějati", addition: "(zasěje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zatelefonovati" => &[
        VerbDictionaryEntry { lemma: "zatelefonovati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zatknųti" => &[
        VerbDictionaryEntry { lemma: "zatknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zatopiti" => &[
        VerbDictionaryEntry { lemma: "zatopiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zatrimati" => &[
        VerbDictionaryEntry { lemma: "zatrimati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zatrimyvati" => &[
        VerbDictionaryEntry { lemma: "zatrimyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zatręsti" => &[
        VerbDictionaryEntry { lemma: "zatręsti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zatvarjati" => &[
        VerbDictionaryEntry { lemma: "zatvarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zatvoriti" => &[
        VerbDictionaryEntry { lemma: "zatvoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zatykati" => &[
        VerbDictionaryEntry { lemma: "zatykati", addition: "(zatyče)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zatėmniti" => &[
        VerbDictionaryEntry { lemma: "zatėmniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zatėmnjati" => &[
        VerbDictionaryEntry { lemma: "zatėmnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zaustaviti" => &[
        VerbDictionaryEntry { lemma: "zaustaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaustavjati" => &[
        VerbDictionaryEntry { lemma: "zaustavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zautrakati" => &[
        VerbDictionaryEntry { lemma: "zautrakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zauvažati" => &[
        VerbDictionaryEntry { lemma: "zauvažati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zauvažiti" => &[
        VerbDictionaryEntry { lemma: "zauvažiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zavaliti" => &[
        VerbDictionaryEntry { lemma: "zavaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaviděti" => &[
        VerbDictionaryEntry { lemma: "zaviděti", addition: "(zavidi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zavladnųti" => &[
        VerbDictionaryEntry { lemma: "zavladnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zavladyvati" => &[
        VerbDictionaryEntry { lemma: "zavladyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zavojevati" => &[
        VerbDictionaryEntry { lemma: "zavojevati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zavojevyvati" => &[
        VerbDictionaryEntry { lemma: "zavojevyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zavraćati" => &[
        VerbDictionaryEntry { lemma: "zavraćati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zavråtiti" => &[
        VerbDictionaryEntry { lemma: "zavråtiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zavyti" => &[
        VerbDictionaryEntry { lemma: "zavyti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zavęzati" => &[
        VerbDictionaryEntry { lemma: "zavęzati", addition: "(zavęže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zavęzyvati" => &[
        VerbDictionaryEntry { lemma: "zavęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zavědyvati" => &[
        VerbDictionaryEntry { lemma: "zavědyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zavěćati" => &[
        VerbDictionaryEntry { lemma: "zavěćati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "zavŕtěti" => &[
        VerbDictionaryEntry { lemma: "zavŕtěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zavŕšati" => &[
        VerbDictionaryEntry { lemma: "zavŕšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zavŕšiti" => &[
        VerbDictionaryEntry { lemma: "zavŕšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "začarovati" => &[
        VerbDictionaryEntry { lemma: "začarovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "začinati" => &[
        VerbDictionaryEntry { lemma: "začinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "začuditi" => &[
        VerbDictionaryEntry { lemma: "začuditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "začęti" => &[
        VerbDictionaryEntry { lemma: "začęti", addition: "(začne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zašifrovati" => &[
        VerbDictionaryEntry { lemma: "zašifrovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zašiti" => &[
        VerbDictionaryEntry { lemma: "zašiti", addition: "(zašije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaštopati" => &[
        VerbDictionaryEntry { lemma: "zaštopati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaščititi" => &[
        VerbDictionaryEntry { lemma: "zaščititi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaščićati" => &[
        VerbDictionaryEntry { lemma: "zaščićati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zaťmiti" => &[
        VerbDictionaryEntry { lemma: "zaťmiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zaťměvati" => &[
        VerbDictionaryEntry { lemma: "zaťměvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zažartovati" => &[
        VerbDictionaryEntry { lemma: "zažartovati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zažegti" => &[
        VerbDictionaryEntry { lemma: "zažegti", addition: "(zažže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zažigati" => &[
        VerbDictionaryEntry { lemma: "zažigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zdråvěti" => &[
        VerbDictionaryEntry { lemma: "zdråvěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zirkati" => &[
        VerbDictionaryEntry { lemma: "zirkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zirknųti" => &[
        VerbDictionaryEntry { lemma: "zirknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "zlobiti" => &[
        VerbDictionaryEntry { lemma: "zlobiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zloupotrěbiti" => &[
        VerbDictionaryEntry { lemma: "zloupotrěbiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "zloupotrěbjati" => &[
        VerbDictionaryEntry { lemma: "zloupotrěbjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zlåtiti" => &[
        VerbDictionaryEntry { lemma: "zlåtiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "znamenovati" => &[
        VerbDictionaryEntry { lemma: "znamenovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "znati" => &[
        VerbDictionaryEntry { lemma: "znati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "značiti" => &[
        VerbDictionaryEntry { lemma: "značiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zrěti" => &[
        VerbDictionaryEntry { lemma: "zrěti", addition: "(zri)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
        VerbDictionaryEntry { lemma: "zrěti", addition: "(zrěje)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zvati" => &[
        VerbDictionaryEntry { lemma: "zvati", addition: "(zȯve)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "zvoniti" => &[
        VerbDictionaryEntry { lemma: "zvoniti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zvěrěti" => &[
        VerbDictionaryEntry { lemma: "zvěrěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zvųčati" => &[
        VerbDictionaryEntry { lemma: "zvųčati", addition: "(zvųči)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zębti" => &[
        VerbDictionaryEntry { lemma: "zębti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zějati" => &[
        VerbDictionaryEntry { lemma: "zějati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zěvati" => &[
        VerbDictionaryEntry { lemma: "zěvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "zěvnųti" => &[
        VerbDictionaryEntry { lemma: "zěvnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true },
    ],
    "čaditi" => &[
        VerbDictionaryEntry { lemma: "čaditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "čarovati" => &[
        VerbDictionaryEntry { lemma: "čarovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "časovati" => &[
        VerbDictionaryEntry { lemma: "časovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "čekati" => &[
        VerbDictionaryEntry { lemma: "čekati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "česati" => &[
        VerbDictionaryEntry { lemma: "česati", addition: "(češe)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "činiti" => &[
        VerbDictionaryEntry { lemma: "činiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "čisliti" => &[
        VerbDictionaryEntry { lemma: "čisliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "čistiti" => &[
        VerbDictionaryEntry { lemma: "čistiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "čitati" => &[
        VerbDictionaryEntry { lemma: "čitati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "črniti" => &[
        VerbDictionaryEntry { lemma: "črniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "črněti" => &[
        VerbDictionaryEntry { lemma: "črněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "črpati" => &[
        VerbDictionaryEntry { lemma: "črpati", addition: "(črpe)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "črstvěti" => &[
        VerbDictionaryEntry { lemma: "črstvěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "črtati" => &[
        VerbDictionaryEntry { lemma: "črtati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "črveněti" => &[
        VerbDictionaryEntry { lemma: "črveněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "čtiti" => &[
        VerbDictionaryEntry { lemma: "čtiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "čuditi" => &[
        VerbDictionaryEntry { lemma: "čuditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "čuti" => &[
        VerbDictionaryEntry { lemma: "čuti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "čuvati" => &[
        VerbDictionaryEntry { lemma: "čuvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "čučati" => &[
        VerbDictionaryEntry { lemma: "čučati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "čėstitati" => &[
        VerbDictionaryEntry { lemma: "čėstitati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false },
    ],
    "čėstiti" => &[
        VerbDictionaryEntry { lemma: "čėstiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "šalěti" => &[
        VerbDictionaryEntry { lemma: "šalěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "šantažovati" => &[
        VerbDictionaryEntry { lemma: "šantažovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "šifrovati" => &[
        VerbDictionaryEntry { lemma: "šifrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "širiti" => &[
        VerbDictionaryEntry { lemma: "širiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "šiti" => &[
        VerbDictionaryEntry { lemma: "šiti", addition: "(šije)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "škoditi" => &[
        VerbDictionaryEntry { lemma: "škoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "školiti" => &[
        VerbDictionaryEntry { lemma: "školiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "šlepati" => &[
        VerbDictionaryEntry { lemma: "šlepati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "šlepnųti" => &[
        VerbDictionaryEntry { lemma: "šlepnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "šlifovati" => &[
        VerbDictionaryEntry { lemma: "šlifovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "šokovati" => &[
        VerbDictionaryEntry { lemma: "šokovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "štopati" => &[
        VerbDictionaryEntry { lemma: "štopati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "šuměti" => &[
        VerbDictionaryEntry { lemma: "šuměti", addition: "(šumi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "ščekotati" => &[
        VerbDictionaryEntry { lemma: "ščekotati", addition: "(ščekoče)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ščipati" => &[
        VerbDictionaryEntry { lemma: "ščipati", addition: "(ščipe)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ščipnųti" => &[
        VerbDictionaryEntry { lemma: "ščipnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "ščititi" => &[
        VerbDictionaryEntry { lemma: "ščititi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "ščęditi" => &[
        VerbDictionaryEntry { lemma: "ščęditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "šėptati" => &[
        VerbDictionaryEntry { lemma: "šėptati", addition: "(šėpće)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false },
    ],
    "žaliti" => &[
        VerbDictionaryEntry { lemma: "žaliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "žariti" => &[
        VerbDictionaryEntry { lemma: "žariti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "žartovati" => &[
        VerbDictionaryEntry { lemma: "žartovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "žegti" => &[
        VerbDictionaryEntry { lemma: "žegti", addition: "(žže)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "želati" => &[
        VerbDictionaryEntry { lemma: "želati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "žiti" => &[
        VerbDictionaryEntry { lemma: "žiti", addition: "(žive)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "žrti" => &[
        VerbDictionaryEntry { lemma: "žrti", addition: "(žre)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "žrtvovati" => &[
        VerbDictionaryEntry { lemma: "žrtvovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "žrěti" => &[
        VerbDictionaryEntry { lemma: "žrěti", addition: "(žre)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "žuvati" => &[
        VerbDictionaryEntry { lemma: "žuvati", addition: "(žuje)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "žužati" => &[
        VerbDictionaryEntry { lemma: "žužati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
    "žędati" => &[
        VerbDictionaryEntry { lemma: "žędati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "žęti" => &[
        VerbDictionaryEntry { lemma: "žęti", addition: "(žne)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false },
    ],
    "žȯltěti" => &[
        VerbDictionaryEntry { lemma: "žȯltěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true },
    ],
};

pub(crate) fn get_verbs(word: &str) -> Option<&'static [VerbDictionaryEntry]> {
    VERB_METADATA.get(word).copied()
}
