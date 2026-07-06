use phf::phf_map;
use super::VerbDictionaryEntry;

pub(crate) static VERB_METADATA: phf::Map<&'static str, &'static [VerbDictionaryEntry]> = phf_map! {
    "#doględati" => &[
        VerbDictionaryEntry { lemma: "#doględati", addition: "", transitive: true, imperfective: true },
    ],
    "#goniti" => &[
        VerbDictionaryEntry { lemma: "#goniti", addition: "", transitive: true, imperfective: true },
    ],
    "#izdělati" => &[
        VerbDictionaryEntry { lemma: "#izdělati", addition: "", transitive: true, imperfective: false },
    ],
    "#izkljuvati" => &[
        VerbDictionaryEntry { lemma: "#izkljuvati", addition: "", transitive: true, imperfective: true },
    ],
    "#izobličati" => &[
        VerbDictionaryEntry { lemma: "#izobličati", addition: "", transitive: true, imperfective: true },
    ],
    "#izstųpiti" => &[
        VerbDictionaryEntry { lemma: "#izstųpiti", addition: "", transitive: true, imperfective: false },
    ],
    "#oblěkati" => &[
        VerbDictionaryEntry { lemma: "#oblěkati", addition: "", transitive: true, imperfective: true },
    ],
    "#obstanoviti" => &[
        VerbDictionaryEntry { lemma: "#obstanoviti", addition: "", transitive: true, imperfective: false },
    ],
    "#ocěnjati" => &[
        VerbDictionaryEntry { lemma: "#ocěnjati", addition: "", transitive: true, imperfective: true },
    ],
    "#odtiskati" => &[
        VerbDictionaryEntry { lemma: "#odtiskati", addition: "", transitive: true, imperfective: true },
    ],
    "#orositi" => &[
        VerbDictionaryEntry { lemma: "#orositi", addition: "", transitive: true, imperfective: false },
    ],
    "#ovoščiti" => &[
        VerbDictionaryEntry { lemma: "#ovoščiti", addition: "", transitive: true, imperfective: false },
    ],
    "#oznaniti" => &[
        VerbDictionaryEntry { lemma: "#oznaniti", addition: "", transitive: true, imperfective: false },
    ],
    "#pnųti" => &[
        VerbDictionaryEntry { lemma: "#pnųti", addition: "", transitive: true, imperfective: false },
    ],
    "#posmotriti" => &[
        VerbDictionaryEntry { lemma: "#posmotriti", addition: "", transitive: true, imperfective: false },
    ],
    "#pretendovati" => &[
        VerbDictionaryEntry { lemma: "#pretendovati", addition: "", transitive: true, imperfective: true },
    ],
    "#prědpostavjati" => &[
        VerbDictionaryEntry { lemma: "#prědpostavjati", addition: "", transitive: true, imperfective: true },
    ],
    "#prěznačati" => &[
        VerbDictionaryEntry { lemma: "#prěznačati", addition: "", transitive: true, imperfective: true },
    ],
    "#råzstlati" => &[
        VerbDictionaryEntry { lemma: "#råzstlati", addition: "(råzstelje)", transitive: true, imperfective: false },
    ],
    "#smotriti" => &[
        VerbDictionaryEntry { lemma: "#smotriti", addition: "", transitive: true, imperfective: true },
    ],
    "#spisati" => &[
        VerbDictionaryEntry { lemma: "#spisati", addition: "(spiše)", transitive: true, imperfective: false },
    ],
    "#stvŕditi" => &[
        VerbDictionaryEntry { lemma: "#stvŕditi", addition: "", transitive: true, imperfective: false },
    ],
    "#sypati" => &[
        VerbDictionaryEntry { lemma: "#sypati", addition: "", transitive: true, imperfective: true },
    ],
    "#sčityvati" => &[
        VerbDictionaryEntry { lemma: "#sčityvati", addition: "", transitive: true, imperfective: true },
    ],
    "#sȯžegti" => &[
        VerbDictionaryEntry { lemma: "#sȯžegti", addition: "(sȯžže)", transitive: true, imperfective: false },
    ],
    "#urězati" => &[
        VerbDictionaryEntry { lemma: "#urězati", addition: "(urěže)", transitive: true, imperfective: false },
    ],
    "#učiniti" => &[
        VerbDictionaryEntry { lemma: "#učiniti", addition: "", transitive: true, imperfective: false },
    ],
    "#vlězti" => &[
        VerbDictionaryEntry { lemma: "#vlězti", addition: "", transitive: true, imperfective: false },
    ],
    "#vsosati" => &[
        VerbDictionaryEntry { lemma: "#vsosati", addition: "", transitive: true, imperfective: false },
    ],
    "#vykati" => &[
        VerbDictionaryEntry { lemma: "#vykati", addition: "", transitive: true, imperfective: true },
    ],
    "#vytŕpěti" => &[
        VerbDictionaryEntry { lemma: "#vytŕpěti", addition: "(vytŕpi)", transitive: true, imperfective: false },
    ],
    "#vyznavati" => &[
        VerbDictionaryEntry { lemma: "#vyznavati", addition: "", transitive: true, imperfective: true },
    ],
    "#vȯzdvigati" => &[
        VerbDictionaryEntry { lemma: "#vȯzdvigati", addition: "", transitive: true, imperfective: true },
    ],
    "#vȯzkypyvati" => &[
        VerbDictionaryEntry { lemma: "#vȯzkypyvati", addition: "", transitive: true, imperfective: true },
    ],
    "#zakazati" => &[
        VerbDictionaryEntry { lemma: "#zakazati", addition: "(zakaže)", transitive: true, imperfective: false },
    ],
    "#zasvětiti" => &[
        VerbDictionaryEntry { lemma: "#zasvětiti", addition: "", transitive: true, imperfective: false },
    ],
    "abdikovati" => &[
        VerbDictionaryEntry { lemma: "abdikovati", addition: "", transitive: true, imperfective: true },
    ],
    "abonovati" => &[
        VerbDictionaryEntry { lemma: "abonovati", addition: "", transitive: true, imperfective: true },
    ],
    "absolutizovati" => &[
        VerbDictionaryEntry { lemma: "absolutizovati", addition: "", transitive: true, imperfective: true },
    ],
    "absorbovati" => &[
        VerbDictionaryEntry { lemma: "absorbovati", addition: "", transitive: true, imperfective: true },
    ],
    "abstrahovati" => &[
        VerbDictionaryEntry { lemma: "abstrahovati", addition: "", transitive: true, imperfective: true },
    ],
    "adaptovati" => &[
        VerbDictionaryEntry { lemma: "adaptovati", addition: "", transitive: true, imperfective: true },
    ],
    "administrovati" => &[
        VerbDictionaryEntry { lemma: "administrovati", addition: "", transitive: true, imperfective: true },
    ],
    "adoptovati" => &[
        VerbDictionaryEntry { lemma: "adoptovati", addition: "", transitive: true, imperfective: true },
    ],
    "adresovati" => &[
        VerbDictionaryEntry { lemma: "adresovati", addition: "", transitive: true, imperfective: true },
    ],
    "agitovati" => &[
        VerbDictionaryEntry { lemma: "agitovati", addition: "", transitive: true, imperfective: true },
    ],
    "agonizovati" => &[
        VerbDictionaryEntry { lemma: "agonizovati", addition: "", transitive: true, imperfective: true },
    ],
    "akcentovati" => &[
        VerbDictionaryEntry { lemma: "akcentovati", addition: "", transitive: true, imperfective: true },
    ],
    "akceptovati" => &[
        VerbDictionaryEntry { lemma: "akceptovati", addition: "", transitive: true, imperfective: true },
    ],
    "aklimatizovati" => &[
        VerbDictionaryEntry { lemma: "aklimatizovati", addition: "", transitive: true, imperfective: true },
    ],
    "akompanovati" => &[
        VerbDictionaryEntry { lemma: "akompanovati", addition: "", transitive: true, imperfective: true },
    ],
    "aktivovati" => &[
        VerbDictionaryEntry { lemma: "aktivovati", addition: "", transitive: true, imperfective: true },
    ],
    "aktualizovati" => &[
        VerbDictionaryEntry { lemma: "aktualizovati", addition: "", transitive: true, imperfective: true },
    ],
    "akumulovati" => &[
        VerbDictionaryEntry { lemma: "akumulovati", addition: "", transitive: true, imperfective: true },
    ],
    "amnestovati" => &[
        VerbDictionaryEntry { lemma: "amnestovati", addition: "", transitive: true, imperfective: true },
    ],
    "amortizovati" => &[
        VerbDictionaryEntry { lemma: "amortizovati", addition: "", transitive: true, imperfective: true },
    ],
    "amputovati" => &[
        VerbDictionaryEntry { lemma: "amputovati", addition: "", transitive: true, imperfective: true },
    ],
    "analizovati" => &[
        VerbDictionaryEntry { lemma: "analizovati", addition: "", transitive: true, imperfective: true },
    ],
    "aneksovati" => &[
        VerbDictionaryEntry { lemma: "aneksovati", addition: "", transitive: true, imperfective: true },
    ],
    "anulovati" => &[
        VerbDictionaryEntry { lemma: "anulovati", addition: "", transitive: true, imperfective: true },
    ],
    "apelovati" => &[
        VerbDictionaryEntry { lemma: "apelovati", addition: "", transitive: true, imperfective: true },
    ],
    "aplodovati" => &[
        VerbDictionaryEntry { lemma: "aplodovati", addition: "", transitive: true, imperfective: true },
    ],
    "aranževati" => &[
        VerbDictionaryEntry { lemma: "aranževati", addition: "", transitive: true, imperfective: true },
    ],
    "areštovati" => &[
        VerbDictionaryEntry { lemma: "areštovati", addition: "", transitive: true, imperfective: true },
    ],
    "argumentovati" => &[
        VerbDictionaryEntry { lemma: "argumentovati", addition: "", transitive: true, imperfective: true },
    ],
    "arhivovati" => &[
        VerbDictionaryEntry { lemma: "arhivovati", addition: "", transitive: true, imperfective: true },
    ],
    "asimilovati" => &[
        VerbDictionaryEntry { lemma: "asimilovati", addition: "", transitive: true, imperfective: true },
    ],
    "atakovati" => &[
        VerbDictionaryEntry { lemma: "atakovati", addition: "", transitive: true, imperfective: true },
    ],
    "avansovati" => &[
        VerbDictionaryEntry { lemma: "avansovati", addition: "", transitive: true, imperfective: true },
    ],
    "avtomatizovati" => &[
        VerbDictionaryEntry { lemma: "avtomatizovati", addition: "", transitive: true, imperfective: true },
    ],
    "avtorizovati" => &[
        VerbDictionaryEntry { lemma: "avtorizovati", addition: "", transitive: true, imperfective: true },
    ],
    "avtostopovati" => &[
        VerbDictionaryEntry { lemma: "avtostopovati", addition: "", transitive: true, imperfective: true },
    ],
    "bagatelizovati" => &[
        VerbDictionaryEntry { lemma: "bagatelizovati", addition: "", transitive: true, imperfective: true },
    ],
    "bajati" => &[
        VerbDictionaryEntry { lemma: "bajati", addition: "", transitive: true, imperfective: true },
    ],
    "balansovati" => &[
        VerbDictionaryEntry { lemma: "balansovati", addition: "", transitive: true, imperfective: true },
    ],
    "balotovati" => &[
        VerbDictionaryEntry { lemma: "balotovati", addition: "", transitive: true, imperfective: true },
    ],
    "balzamovati" => &[
        VerbDictionaryEntry { lemma: "balzamovati", addition: "", transitive: true, imperfective: true },
    ],
    "barikadovati" => &[
        VerbDictionaryEntry { lemma: "barikadovati", addition: "", transitive: true, imperfective: true },
    ],
    "barviti" => &[
        VerbDictionaryEntry { lemma: "barviti", addition: "", transitive: true, imperfective: true },
    ],
    "baviti" => &[
        VerbDictionaryEntry { lemma: "baviti", addition: "", transitive: true, imperfective: true },
    ],
    "bděti" => &[
        VerbDictionaryEntry { lemma: "bděti", addition: "(bdi)", transitive: true, imperfective: true },
    ],
    "besědovati" => &[
        VerbDictionaryEntry { lemma: "besědovati", addition: "", transitive: true, imperfective: true },
    ],
    "bezpokojiti" => &[
        VerbDictionaryEntry { lemma: "bezpokojiti", addition: "", transitive: true, imperfective: true },
    ],
    "biti" => &[
        VerbDictionaryEntry { lemma: "biti", addition: "(bije)", transitive: true, imperfective: true },
    ],
    "bičevati" => &[
        VerbDictionaryEntry { lemma: "bičevati", addition: "", transitive: true, imperfective: true },
    ],
    "blaznovati" => &[
        VerbDictionaryEntry { lemma: "blaznovati", addition: "", transitive: true, imperfective: true },
    ],
    "blejati" => &[
        VerbDictionaryEntry { lemma: "blejati", addition: "", transitive: true, imperfective: true },
    ],
    "bljunųti" => &[
        VerbDictionaryEntry { lemma: "bljunųti", addition: "", transitive: true, imperfective: false },
    ],
    "bljuvati" => &[
        VerbDictionaryEntry { lemma: "bljuvati", addition: "", transitive: true, imperfective: true },
    ],
    "blokovati" => &[
        VerbDictionaryEntry { lemma: "blokovati", addition: "", transitive: true, imperfective: true },
    ],
    "blågodariti" => &[
        VerbDictionaryEntry { lemma: "blågodariti", addition: "", transitive: true, imperfective: true },
    ],
    "blågoslavjati" => &[
        VerbDictionaryEntry { lemma: "blågoslavjati", addition: "", transitive: true, imperfective: true },
    ],
    "blågosloviti" => &[
        VerbDictionaryEntry { lemma: "blågosloviti", addition: "", transitive: true, imperfective: false },
    ],
    "blågovolěti" => &[
        VerbDictionaryEntry { lemma: "blågovolěti", addition: "(blågovoli)", transitive: true, imperfective: true },
    ],
    "blågoželati" => &[
        VerbDictionaryEntry { lemma: "blågoželati", addition: "(+3)", transitive: true, imperfective: true },
    ],
    "blědněti" => &[
        VerbDictionaryEntry { lemma: "blědněti", addition: "", transitive: true, imperfective: true },
    ],
    "blěskati" => &[
        VerbDictionaryEntry { lemma: "blěskati", addition: "", transitive: true, imperfective: true },
    ],
    "blěsknųti" => &[
        VerbDictionaryEntry { lemma: "blěsknųti", addition: "", transitive: true, imperfective: false },
    ],
    "blěstěti" => &[
        VerbDictionaryEntry { lemma: "blěstěti", addition: "", transitive: true, imperfective: true },
    ],
    "blųditi" => &[
        VerbDictionaryEntry { lemma: "blųditi", addition: "", transitive: true, imperfective: true },
    ],
    "blųkati" => &[
        VerbDictionaryEntry { lemma: "blųkati", addition: "", transitive: true, imperfective: true },
    ],
    "bodati" => &[
        VerbDictionaryEntry { lemma: "bodati", addition: "", transitive: true, imperfective: true },
    ],
    "bodnųti" => &[
        VerbDictionaryEntry { lemma: "bodnųti", addition: "(bode)", transitive: true, imperfective: false },
    ],
    "bogatěti" => &[
        VerbDictionaryEntry { lemma: "bogatěti", addition: "", transitive: true, imperfective: true },
    ],
    "bogohuliti" => &[
        VerbDictionaryEntry { lemma: "bogohuliti", addition: "", transitive: true, imperfective: true },
    ],
    "bojevati" => &[
        VerbDictionaryEntry { lemma: "bojevati", addition: "", transitive: true, imperfective: true },
    ],
    "bojkotovati" => &[
        VerbDictionaryEntry { lemma: "bojkotovati", addition: "", transitive: true, imperfective: true },
    ],
    "bolěti" => &[
        VerbDictionaryEntry { lemma: "bolěti", addition: "(bolěje)", transitive: true, imperfective: true },
        VerbDictionaryEntry { lemma: "bolěti", addition: "(boli)", transitive: true, imperfective: true },
    ],
    "bombardovati" => &[
        VerbDictionaryEntry { lemma: "bombardovati", addition: "", transitive: true, imperfective: true },
    ],
    "bosti" => &[
        VerbDictionaryEntry { lemma: "bosti", addition: "(bode)", transitive: true, imperfective: true },
    ],
    "brati" => &[
        VerbDictionaryEntry { lemma: "brati", addition: "(bere)", transitive: true, imperfective: true },
    ],
    "breknųti" => &[
        VerbDictionaryEntry { lemma: "breknųti", addition: "", transitive: true, imperfective: true },
    ],
    "brenknųti" => &[
        VerbDictionaryEntry { lemma: "brenknųti", addition: "", transitive: true, imperfective: true },
    ],
    "brenčati" => &[
        VerbDictionaryEntry { lemma: "brenčati", addition: "", transitive: true, imperfective: true },
    ],
    "briti" => &[
        VerbDictionaryEntry { lemma: "briti", addition: "(brije)", transitive: true, imperfective: true },
    ],
    "broditi" => &[
        VerbDictionaryEntry { lemma: "broditi", addition: "", transitive: true, imperfective: true },
    ],
    "brusiti" => &[
        VerbDictionaryEntry { lemma: "brusiti", addition: "", transitive: true, imperfective: true },
    ],
    "bryzgati" => &[
        VerbDictionaryEntry { lemma: "bryzgati", addition: "", transitive: true, imperfective: true },
    ],
    "bryzgnųti" => &[
        VerbDictionaryEntry { lemma: "bryzgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "bråniti" => &[
        VerbDictionaryEntry { lemma: "bråniti", addition: "", transitive: true, imperfective: true },
    ],
    "bråzditi" => &[
        VerbDictionaryEntry { lemma: "bråzditi", addition: "", transitive: true, imperfective: true },
    ],
    "buditi" => &[
        VerbDictionaryEntry { lemma: "buditi", addition: "", transitive: true, imperfective: true },
    ],
    "budovati" => &[
        VerbDictionaryEntry { lemma: "budovati", addition: "", transitive: true, imperfective: true },
    ],
    "buhati" => &[
        VerbDictionaryEntry { lemma: "buhati", addition: "", transitive: true, imperfective: true },
    ],
    "buhnųti" => &[
        VerbDictionaryEntry { lemma: "buhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "buriti" => &[
        VerbDictionaryEntry { lemma: "buriti", addition: "", transitive: true, imperfective: true },
    ],
    "bučati" => &[
        VerbDictionaryEntry { lemma: "bučati", addition: "(buče)", transitive: true, imperfective: true },
    ],
    "byti" => &[
        VerbDictionaryEntry { lemma: "byti", addition: "(jesm, jesi, jest, jesmo, jeste, sųt; byl; bųdų)", transitive: true, imperfective: true },
    ],
    "byvati" => &[
        VerbDictionaryEntry { lemma: "byvati", addition: "", transitive: true, imperfective: true },
    ],
    "bzdnųti" => &[
        VerbDictionaryEntry { lemma: "bzdnųti", addition: "", transitive: true, imperfective: false },
    ],
    "bzděti" => &[
        VerbDictionaryEntry { lemma: "bzděti", addition: "", transitive: true, imperfective: true },
    ],
    "běgati" => &[
        VerbDictionaryEntry { lemma: "běgati", addition: "", transitive: true, imperfective: true },
    ],
    "běgti" => &[
        VerbDictionaryEntry { lemma: "běgti", addition: "(běži)", transitive: true, imperfective: true },
    ],
    "běliti" => &[
        VerbDictionaryEntry { lemma: "běliti", addition: "", transitive: true, imperfective: true },
    ],
    "bělěti" => &[
        VerbDictionaryEntry { lemma: "bělěti", addition: "", transitive: true, imperfective: true },
    ],
    "cenzurovati" => &[
        VerbDictionaryEntry { lemma: "cenzurovati", addition: "", transitive: true, imperfective: true },
    ],
    "citovati" => &[
        VerbDictionaryEntry { lemma: "citovati", addition: "", transitive: true, imperfective: true },
    ],
    "cmokati" => &[
        VerbDictionaryEntry { lemma: "cmokati", addition: "", transitive: true, imperfective: true },
    ],
    "cviliti" => &[
        VerbDictionaryEntry { lemma: "cviliti", addition: "", transitive: true, imperfective: true },
    ],
    "cvěliti" => &[
        VerbDictionaryEntry { lemma: "cvěliti", addition: "", transitive: true, imperfective: true },
    ],
    "cvěsti" => &[
        VerbDictionaryEntry { lemma: "cvěsti", addition: "(cvěte)", transitive: true, imperfective: true },
    ],
    "cvětati" => &[
        VerbDictionaryEntry { lemma: "cvětati", addition: "", transitive: true, imperfective: true },
    ],
    "cvětnųti" => &[
        VerbDictionaryEntry { lemma: "cvětnųti", addition: "", transitive: true, imperfective: true },
    ],
    "cvŕkati" => &[
        VerbDictionaryEntry { lemma: "cvŕkati", addition: "", transitive: true, imperfective: true },
    ],
    "cěditi" => &[
        VerbDictionaryEntry { lemma: "cěditi", addition: "", transitive: true, imperfective: true },
    ],
    "cěliti" => &[
        VerbDictionaryEntry { lemma: "cěliti", addition: "", transitive: true, imperfective: true },
    ],
    "cělovati" => &[
        VerbDictionaryEntry { lemma: "cělovati", addition: "", transitive: true, imperfective: true },
    ],
    "cěniti" => &[
        VerbDictionaryEntry { lemma: "cěniti", addition: "", transitive: true, imperfective: true },
    ],
    "dariti" => &[
        VerbDictionaryEntry { lemma: "dariti", addition: "", transitive: true, imperfective: true },
    ],
    "darovati" => &[
        VerbDictionaryEntry { lemma: "darovati", addition: "", transitive: true, imperfective: true },
    ],
    "dati" => &[
        VerbDictionaryEntry { lemma: "dati", addition: "(da)", transitive: true, imperfective: false },
    ],
    "datovati" => &[
        VerbDictionaryEntry { lemma: "datovati", addition: "", transitive: true, imperfective: true },
    ],
    "daunlodovati" => &[
        VerbDictionaryEntry { lemma: "daunlodovati", addition: "", transitive: true, imperfective: true },
    ],
    "davati" => &[
        VerbDictionaryEntry { lemma: "davati", addition: "", transitive: true, imperfective: true },
    ],
    "daviti" => &[
        VerbDictionaryEntry { lemma: "daviti", addition: "", transitive: true, imperfective: true },
    ],
    "dbati" => &[
        VerbDictionaryEntry { lemma: "dbati", addition: "", transitive: true, imperfective: true },
    ],
    "deaktivovati" => &[
        VerbDictionaryEntry { lemma: "deaktivovati", addition: "", transitive: true, imperfective: true },
    ],
    "debatovati" => &[
        VerbDictionaryEntry { lemma: "debatovati", addition: "", transitive: true, imperfective: true },
    ],
    "debelěti" => &[
        VerbDictionaryEntry { lemma: "debelěti", addition: "", transitive: true, imperfective: true },
    ],
    "debjutovati" => &[
        VerbDictionaryEntry { lemma: "debjutovati", addition: "", transitive: true, imperfective: true },
    ],
    "deblokovati" => &[
        VerbDictionaryEntry { lemma: "deblokovati", addition: "", transitive: true, imperfective: true },
    ],
    "definiovati" => &[
        VerbDictionaryEntry { lemma: "definiovati", addition: "", transitive: true, imperfective: true },
    ],
    "defisovati" => &[
        VerbDictionaryEntry { lemma: "defisovati", addition: "", transitive: true, imperfective: true },
    ],
    "deformovati" => &[
        VerbDictionaryEntry { lemma: "deformovati", addition: "", transitive: true, imperfective: true },
    ],
    "degenerovati" => &[
        VerbDictionaryEntry { lemma: "degenerovati", addition: "", transitive: true, imperfective: true },
    ],
    "degradovati" => &[
        VerbDictionaryEntry { lemma: "degradovati", addition: "", transitive: true, imperfective: true },
    ],
    "degustovati" => &[
        VerbDictionaryEntry { lemma: "degustovati", addition: "", transitive: true, imperfective: true },
    ],
    "deklamovati" => &[
        VerbDictionaryEntry { lemma: "deklamovati", addition: "", transitive: true, imperfective: true },
    ],
    "dekodovati" => &[
        VerbDictionaryEntry { lemma: "dekodovati", addition: "", transitive: true, imperfective: true },
    ],
    "demonstrovati" => &[
        VerbDictionaryEntry { lemma: "demonstrovati", addition: "", transitive: true, imperfective: true },
    ],
    "demoralizovati" => &[
        VerbDictionaryEntry { lemma: "demoralizovati", addition: "", transitive: true, imperfective: false },
    ],
    "deportovati" => &[
        VerbDictionaryEntry { lemma: "deportovati", addition: "", transitive: true, imperfective: true },
    ],
    "deprimovati" => &[
        VerbDictionaryEntry { lemma: "deprimovati", addition: "", transitive: true, imperfective: true },
    ],
    "destabilizovati" => &[
        VerbDictionaryEntry { lemma: "destabilizovati", addition: "", transitive: true, imperfective: true },
    ],
    "dezertovati" => &[
        VerbDictionaryEntry { lemma: "dezertovati", addition: "", transitive: true, imperfective: true },
    ],
    "dezinfikovati" => &[
        VerbDictionaryEntry { lemma: "dezinfikovati", addition: "", transitive: true, imperfective: true },
    ],
    "diktovati" => &[
        VerbDictionaryEntry { lemma: "diktovati", addition: "", transitive: true, imperfective: true },
    ],
    "diskreditovati" => &[
        VerbDictionaryEntry { lemma: "diskreditovati", addition: "", transitive: true, imperfective: true },
    ],
    "diskriminovati" => &[
        VerbDictionaryEntry { lemma: "diskriminovati", addition: "", transitive: true, imperfective: true },
    ],
    "diskutovati" => &[
        VerbDictionaryEntry { lemma: "diskutovati", addition: "", transitive: true, imperfective: true },
    ],
    "diskvalifikovati" => &[
        VerbDictionaryEntry { lemma: "diskvalifikovati", addition: "", transitive: true, imperfective: true },
    ],
    "diviti" => &[
        VerbDictionaryEntry { lemma: "diviti", addition: "", transitive: true, imperfective: true },
    ],
    "dobaviti" => &[
        VerbDictionaryEntry { lemma: "dobaviti", addition: "", transitive: true, imperfective: false },
    ],
    "dobavjati" => &[
        VerbDictionaryEntry { lemma: "dobavjati", addition: "", transitive: true, imperfective: true },
    ],
    "dobrěti" => &[
        VerbDictionaryEntry { lemma: "dobrěti", addition: "", transitive: true, imperfective: true },
    ],
    "dobyti" => &[
        VerbDictionaryEntry { lemma: "dobyti", addition: "(dobųde)", transitive: true, imperfective: false },
    ],
    "dobyvati" => &[
        VerbDictionaryEntry { lemma: "dobyvati", addition: "", transitive: true, imperfective: true },
    ],
    "dodati" => &[
        VerbDictionaryEntry { lemma: "dodati", addition: "(doda)", transitive: true, imperfective: false },
    ],
    "dodavati" => &[
        VerbDictionaryEntry { lemma: "dodavati", addition: "", transitive: true, imperfective: true },
    ],
    "doganjati" => &[
        VerbDictionaryEntry { lemma: "doganjati", addition: "", transitive: true, imperfective: true },
    ],
    "dogarjati" => &[
        VerbDictionaryEntry { lemma: "dogarjati", addition: "", transitive: true, imperfective: true },
    ],
    "doględěti" => &[
        VerbDictionaryEntry { lemma: "doględěti", addition: "(doględi)", transitive: true, imperfective: false },
    ],
    "dognati" => &[
        VerbDictionaryEntry { lemma: "dognati", addition: "(dogone)", transitive: true, imperfective: false },
    ],
    "dogorěti" => &[
        VerbDictionaryEntry { lemma: "dogorěti", addition: "", transitive: true, imperfective: false },
    ],
    "dogovarjati" => &[
        VerbDictionaryEntry { lemma: "dogovarjati", addition: "", transitive: true, imperfective: true },
    ],
    "dogovoriti" => &[
        VerbDictionaryEntry { lemma: "dogovoriti", addition: "", transitive: true, imperfective: false },
    ],
    "dohoditi" => &[
        VerbDictionaryEntry { lemma: "dohoditi", addition: "", transitive: true, imperfective: true },
    ],
    "dojdti" => &[
        VerbDictionaryEntry { lemma: "dojdti", addition: "(dojde; došėl)", transitive: true, imperfective: false },
    ],
    "dojehati" => &[
        VerbDictionaryEntry { lemma: "dojehati", addition: "(dojede)", transitive: true, imperfective: false },
    ],
    "doježđati" => &[
        VerbDictionaryEntry { lemma: "doježđati", addition: "", transitive: true, imperfective: true },
    ],
    "dojiti" => &[
        VerbDictionaryEntry { lemma: "dojiti", addition: "", transitive: true, imperfective: true },
    ],
    "dokazati" => &[
        VerbDictionaryEntry { lemma: "dokazati", addition: "(dokaže)", transitive: true, imperfective: false },
    ],
    "dokazyvati" => &[
        VerbDictionaryEntry { lemma: "dokazyvati", addition: "", transitive: true, imperfective: true },
    ],
    "dokladati" => &[
        VerbDictionaryEntry { lemma: "dokladati", addition: "", transitive: true, imperfective: true },
    ],
    "dokonati" => &[
        VerbDictionaryEntry { lemma: "dokonati", addition: "", transitive: true, imperfective: false },
    ],
    "dokonyvati" => &[
        VerbDictionaryEntry { lemma: "dokonyvati", addition: "", transitive: true, imperfective: true },
    ],
    "dokončiti" => &[
        VerbDictionaryEntry { lemma: "dokončiti", addition: "", transitive: true, imperfective: false },
    ],
    "dokumentovati" => &[
        VerbDictionaryEntry { lemma: "dokumentovati", addition: "", transitive: true, imperfective: true },
    ],
    "dokupiti" => &[
        VerbDictionaryEntry { lemma: "dokupiti", addition: "", transitive: true, imperfective: false },
    ],
    "dokupovati" => &[
        VerbDictionaryEntry { lemma: "dokupovati", addition: "", transitive: true, imperfective: true },
    ],
    "doletěti" => &[
        VerbDictionaryEntry { lemma: "doletěti", addition: "", transitive: true, imperfective: false },
    ],
    "doložiti" => &[
        VerbDictionaryEntry { lemma: "doložiti", addition: "", transitive: true, imperfective: false },
    ],
    "dominovati" => &[
        VerbDictionaryEntry { lemma: "dominovati", addition: "", transitive: true, imperfective: true },
    ],
    "domněvati" => &[
        VerbDictionaryEntry { lemma: "domněvati", addition: "", transitive: true, imperfective: true },
    ],
    "donesti" => &[
        VerbDictionaryEntry { lemma: "donesti", addition: "", transitive: true, imperfective: false },
    ],
    "donositi" => &[
        VerbDictionaryEntry { lemma: "donositi", addition: "", transitive: true, imperfective: true },
    ],
    "dopisati" => &[
        VerbDictionaryEntry { lemma: "dopisati", addition: "", transitive: true, imperfective: false },
    ],
    "dopisyvati" => &[
        VerbDictionaryEntry { lemma: "dopisyvati", addition: "", transitive: true, imperfective: true },
    ],
    "doplatiti" => &[
        VerbDictionaryEntry { lemma: "doplatiti", addition: "", transitive: true, imperfective: false },
    ],
    "doplaćati" => &[
        VerbDictionaryEntry { lemma: "doplaćati", addition: "", transitive: true, imperfective: true },
    ],
    "dopustiti" => &[
        VerbDictionaryEntry { lemma: "dopustiti", addition: "", transitive: true, imperfective: false },
    ],
    "dopušćati" => &[
        VerbDictionaryEntry { lemma: "dopušćati", addition: "", transitive: true, imperfective: true },
    ],
    "dopȯlniti" => &[
        VerbDictionaryEntry { lemma: "dopȯlniti", addition: "", transitive: true, imperfective: false },
    ],
    "dopȯlnjati" => &[
        VerbDictionaryEntry { lemma: "dopȯlnjati", addition: "", transitive: true, imperfective: true },
    ],
    "dopȯlzati" => &[
        VerbDictionaryEntry { lemma: "dopȯlzati", addition: "", transitive: true, imperfective: true },
    ],
    "dopȯlzti" => &[
        VerbDictionaryEntry { lemma: "dopȯlzti", addition: "", transitive: true, imperfective: false },
    ],
    "dorastati" => &[
        VerbDictionaryEntry { lemma: "dorastati", addition: "", transitive: true, imperfective: true },
    ],
    "dorysovati" => &[
        VerbDictionaryEntry { lemma: "dorysovati", addition: "", transitive: true, imperfective: false },
    ],
    "dorysovyvati" => &[
        VerbDictionaryEntry { lemma: "dorysovyvati", addition: "", transitive: true, imperfective: true },
    ],
    "doråsti" => &[
        VerbDictionaryEntry { lemma: "doråsti", addition: "(doråste)", transitive: true, imperfective: false },
    ],
    "dorųčati" => &[
        VerbDictionaryEntry { lemma: "dorųčati", addition: "", transitive: true, imperfective: true },
    ],
    "dorųčiti" => &[
        VerbDictionaryEntry { lemma: "dorųčiti", addition: "", transitive: true, imperfective: false },
    ],
    "dosaditi" => &[
        VerbDictionaryEntry { lemma: "dosaditi", addition: "", transitive: true, imperfective: false },
    ],
    "dosađati" => &[
        VerbDictionaryEntry { lemma: "dosađati", addition: "", transitive: true, imperfective: true },
    ],
    "doskonaliti" => &[
        VerbDictionaryEntry { lemma: "doskonaliti", addition: "", transitive: true, imperfective: true },
    ],
    "dostati" => &[
        VerbDictionaryEntry { lemma: "dostati", addition: "(dostane)", transitive: true, imperfective: false },
    ],
    "dostavati" => &[
        VerbDictionaryEntry { lemma: "dostavati", addition: "", transitive: true, imperfective: true },
    ],
    "dostaviti" => &[
        VerbDictionaryEntry { lemma: "dostaviti", addition: "", transitive: true, imperfective: false },
    ],
    "dostavjati" => &[
        VerbDictionaryEntry { lemma: "dostavjati", addition: "", transitive: true, imperfective: true },
    ],
    "dostigati" => &[
        VerbDictionaryEntry { lemma: "dostigati", addition: "", transitive: true, imperfective: true },
    ],
    "dostignųti" => &[
        VerbDictionaryEntry { lemma: "dostignųti", addition: "", transitive: true, imperfective: false },
    ],
    "dosęgati" => &[
        VerbDictionaryEntry { lemma: "dosęgati", addition: "", transitive: true, imperfective: true },
    ],
    "dosęgnųti" => &[
        VerbDictionaryEntry { lemma: "dosęgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "dotknųti" => &[
        VerbDictionaryEntry { lemma: "dotknųti", addition: "", transitive: true, imperfective: false },
    ],
    "dotykati" => &[
        VerbDictionaryEntry { lemma: "dotykati", addition: "(dotyče)", transitive: true, imperfective: true },
    ],
    "dovezti" => &[
        VerbDictionaryEntry { lemma: "dovezti", addition: "", transitive: true, imperfective: false },
    ],
    "dovoziti" => &[
        VerbDictionaryEntry { lemma: "dovoziti", addition: "", transitive: true, imperfective: true },
    ],
    "dověriti" => &[
        VerbDictionaryEntry { lemma: "dověriti", addition: "", transitive: true, imperfective: false },
    ],
    "dověrjati" => &[
        VerbDictionaryEntry { lemma: "dověrjati", addition: "", transitive: true, imperfective: true },
    ],
    "dovŕšati" => &[
        VerbDictionaryEntry { lemma: "dovŕšati", addition: "", transitive: true, imperfective: true },
    ],
    "dovŕšiti" => &[
        VerbDictionaryEntry { lemma: "dovŕšiti", addition: "", transitive: true, imperfective: false },
    ],
    "dozovati" => &[
        VerbDictionaryEntry { lemma: "dozovati", addition: "", transitive: true, imperfective: true },
    ],
    "dozrěti" => &[
        VerbDictionaryEntry { lemma: "dozrěti", addition: "", transitive: true, imperfective: false },
    ],
    "dozrěvati" => &[
        VerbDictionaryEntry { lemma: "dozrěvati", addition: "", transitive: true, imperfective: true },
    ],
    "dozvaljati" => &[
        VerbDictionaryEntry { lemma: "dozvaljati", addition: "", transitive: true, imperfective: true },
    ],
    "dozvoliti" => &[
        VerbDictionaryEntry { lemma: "dozvoliti", addition: "", transitive: true, imperfective: false },
    ],
    "dožiti" => &[
        VerbDictionaryEntry { lemma: "dožiti", addition: "(dožive)", transitive: true, imperfective: false },
    ],
    "doživati" => &[
        VerbDictionaryEntry { lemma: "doživati", addition: "", transitive: true, imperfective: true },
    ],
    "drapati" => &[
        VerbDictionaryEntry { lemma: "drapati", addition: "(drape)", transitive: true, imperfective: true },
    ],
    "drapnųti" => &[
        VerbDictionaryEntry { lemma: "drapnųti", addition: "", transitive: true, imperfective: false },
    ],
    "dražniti" => &[
        VerbDictionaryEntry { lemma: "dražniti", addition: "", transitive: true, imperfective: true },
    ],
    "drgati" => &[
        VerbDictionaryEntry { lemma: "drgati", addition: "", transitive: true, imperfective: true },
    ],
    "drgnųti" => &[
        VerbDictionaryEntry { lemma: "drgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "drobiti" => &[
        VerbDictionaryEntry { lemma: "drobiti", addition: "", transitive: true, imperfective: true },
    ],
    "drěmati" => &[
        VerbDictionaryEntry { lemma: "drěmati", addition: "(drěme)", transitive: true, imperfective: true },
    ],
    "drěmnųti" => &[
        VerbDictionaryEntry { lemma: "drěmnųti", addition: "", transitive: true, imperfective: false },
    ],
    "drěti" => &[
        VerbDictionaryEntry { lemma: "drěti", addition: "(dre)", transitive: true, imperfective: true },
    ],
    "držati" => &[
        VerbDictionaryEntry { lemma: "držati", addition: "(drži)", transitive: true, imperfective: true },
    ],
    "dubliti" => &[
        VerbDictionaryEntry { lemma: "dubliti", addition: "", transitive: true, imperfective: true },
    ],
    "dumati" => &[
        VerbDictionaryEntry { lemma: "dumati", addition: "", transitive: true, imperfective: true },
    ],
    "durěti" => &[
        VerbDictionaryEntry { lemma: "durěti", addition: "", transitive: true, imperfective: true },
    ],
    "dušiti" => &[
        VerbDictionaryEntry { lemma: "dušiti", addition: "", transitive: true, imperfective: true },
    ],
    "dvigati" => &[
        VerbDictionaryEntry { lemma: "dvigati", addition: "", transitive: true, imperfective: true },
    ],
    "dvignųti" => &[
        VerbDictionaryEntry { lemma: "dvignųti", addition: "", transitive: true, imperfective: false },
    ],
    "dvojiti" => &[
        VerbDictionaryEntry { lemma: "dvojiti", addition: "", transitive: true, imperfective: true },
    ],
    "dyhati" => &[
        VerbDictionaryEntry { lemma: "dyhati", addition: "", transitive: true, imperfective: true },
    ],
    "dyhtěti" => &[
        VerbDictionaryEntry { lemma: "dyhtěti", addition: "(dyhti)", transitive: true, imperfective: true },
    ],
    "dymiti" => &[
        VerbDictionaryEntry { lemma: "dymiti", addition: "", transitive: true, imperfective: true },
    ],
    "dyšati" => &[
        VerbDictionaryEntry { lemma: "dyšati", addition: "(dyše)", transitive: true, imperfective: true },
    ],
    "dękovati" => &[
        VerbDictionaryEntry { lemma: "dękovati", addition: "(+3)", transitive: true, imperfective: true },
    ],
    "dějati" => &[
        VerbDictionaryEntry { lemma: "dějati", addition: "(děje)", transitive: true, imperfective: true },
    ],
    "dělati" => &[
        VerbDictionaryEntry { lemma: "dělati", addition: "", transitive: true, imperfective: true },
    ],
    "děliti" => &[
        VerbDictionaryEntry { lemma: "děliti", addition: "", transitive: true, imperfective: true },
    ],
    "děti" => &[
        VerbDictionaryEntry { lemma: "děti", addition: "(děje/děne)", transitive: true, imperfective: true },
    ],
    "dŕkati" => &[
        VerbDictionaryEntry { lemma: "dŕkati", addition: "", transitive: true, imperfective: true },
    ],
    "dŕti" => &[
        VerbDictionaryEntry { lemma: "dŕti", addition: "(dre)", transitive: true, imperfective: true },
    ],
    "dŕzati" => &[
        VerbDictionaryEntry { lemma: "dŕzati", addition: "", transitive: true, imperfective: true },
    ],
    "dŕznųti" => &[
        VerbDictionaryEntry { lemma: "dŕznųti", addition: "", transitive: true, imperfective: false },
    ],
    "dŕžati" => &[
        VerbDictionaryEntry { lemma: "dŕžati", addition: "(dŕži)", transitive: true, imperfective: true },
    ],
    "dųbiti" => &[
        VerbDictionaryEntry { lemma: "dųbiti", addition: "", transitive: true, imperfective: true },
    ],
    "dųnųti" => &[
        VerbDictionaryEntry { lemma: "dųnųti", addition: "", transitive: true, imperfective: false },
    ],
    "dųti" => &[
        VerbDictionaryEntry { lemma: "dųti", addition: "(dme)", transitive: true, imperfective: true },
    ],
    "dȯhnųti" => &[
        VerbDictionaryEntry { lemma: "dȯhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "dȯlbti" => &[
        VerbDictionaryEntry { lemma: "dȯlbti", addition: "(dȯlbe)", transitive: true, imperfective: true },
    ],
    "dȯlgočasiti" => &[
        VerbDictionaryEntry { lemma: "dȯlgočasiti", addition: "", transitive: true, imperfective: true },
    ],
    "dȯlžiti" => &[
        VerbDictionaryEntry { lemma: "dȯlžiti", addition: "", transitive: true, imperfective: true },
    ],
    "dȯžditi" => &[
        VerbDictionaryEntry { lemma: "dȯžditi", addition: "", transitive: true, imperfective: true },
    ],
    "egzistovati" => &[
        VerbDictionaryEntry { lemma: "egzistovati", addition: "", transitive: true, imperfective: true },
    ],
    "ejakulovati" => &[
        VerbDictionaryEntry { lemma: "ejakulovati", addition: "", transitive: true, imperfective: true },
    ],
    "eksperimentovati" => &[
        VerbDictionaryEntry { lemma: "eksperimentovati", addition: "", transitive: true, imperfective: true },
    ],
    "eksploatovati" => &[
        VerbDictionaryEntry { lemma: "eksploatovati", addition: "", transitive: true, imperfective: true },
    ],
    "eksplodovati" => &[
        VerbDictionaryEntry { lemma: "eksplodovati", addition: "", transitive: true, imperfective: true },
    ],
    "eksportovati" => &[
        VerbDictionaryEntry { lemma: "eksportovati", addition: "", transitive: true, imperfective: true },
    ],
    "ekstragovati" => &[
        VerbDictionaryEntry { lemma: "ekstragovati", addition: "", transitive: true, imperfective: true },
    ],
    "eliminovati" => &[
        VerbDictionaryEntry { lemma: "eliminovati", addition: "", transitive: true, imperfective: true },
    ],
    "emigrovati" => &[
        VerbDictionaryEntry { lemma: "emigrovati", addition: "", transitive: true, imperfective: true },
    ],
    "entuziazmovati" => &[
        VerbDictionaryEntry { lemma: "entuziazmovati", addition: "", transitive: true, imperfective: true },
    ],
    "eskalovati" => &[
        VerbDictionaryEntry { lemma: "eskalovati", addition: "", transitive: true, imperfective: true },
    ],
    "eskortovati" => &[
        VerbDictionaryEntry { lemma: "eskortovati", addition: "", transitive: true, imperfective: true },
    ],
    "evakuovati" => &[
        VerbDictionaryEntry { lemma: "evakuovati", addition: "", transitive: true, imperfective: true },
    ],
    "fabrikovati" => &[
        VerbDictionaryEntry { lemma: "fabrikovati", addition: "", transitive: true, imperfective: true },
    ],
    "falsifikovati" => &[
        VerbDictionaryEntry { lemma: "falsifikovati", addition: "", transitive: true, imperfective: true },
    ],
    "farbovati" => &[
        VerbDictionaryEntry { lemma: "farbovati", addition: "", transitive: true, imperfective: true },
    ],
    "filmovati" => &[
        VerbDictionaryEntry { lemma: "filmovati", addition: "", transitive: true, imperfective: true },
    ],
    "filtrovati" => &[
        VerbDictionaryEntry { lemma: "filtrovati", addition: "", transitive: true, imperfective: true },
    ],
    "finansovati" => &[
        VerbDictionaryEntry { lemma: "finansovati", addition: "", transitive: true, imperfective: true },
    ],
    "flirtovati" => &[
        VerbDictionaryEntry { lemma: "flirtovati", addition: "", transitive: true, imperfective: true },
    ],
    "formalizovati" => &[
        VerbDictionaryEntry { lemma: "formalizovati", addition: "", transitive: true, imperfective: true },
    ],
    "formovati" => &[
        VerbDictionaryEntry { lemma: "formovati", addition: "", transitive: true, imperfective: true },
    ],
    "formulovati" => &[
        VerbDictionaryEntry { lemma: "formulovati", addition: "", transitive: true, imperfective: true },
    ],
    "fotografovati" => &[
        VerbDictionaryEntry { lemma: "fotografovati", addition: "", transitive: true, imperfective: true },
    ],
    "frustrovati" => &[
        VerbDictionaryEntry { lemma: "frustrovati", addition: "", transitive: true, imperfective: true },
    ],
    "funkcionovati" => &[
        VerbDictionaryEntry { lemma: "funkcionovati", addition: "", transitive: true, imperfective: true },
    ],
    "gadati" => &[
        VerbDictionaryEntry { lemma: "gadati", addition: "", transitive: true, imperfective: true },
    ],
    "galopovati" => &[
        VerbDictionaryEntry { lemma: "galopovati", addition: "", transitive: true, imperfective: true },
    ],
    "galvanizovati" => &[
        VerbDictionaryEntry { lemma: "galvanizovati", addition: "", transitive: true, imperfective: true },
    ],
    "garantovati" => &[
        VerbDictionaryEntry { lemma: "garantovati", addition: "", transitive: true, imperfective: true },
    ],
    "garnirovati" => &[
        VerbDictionaryEntry { lemma: "garnirovati", addition: "", transitive: true, imperfective: true },
    ],
    "gasiti" => &[
        VerbDictionaryEntry { lemma: "gasiti", addition: "", transitive: true, imperfective: true },
    ],
    "gasnųti" => &[
        VerbDictionaryEntry { lemma: "gasnųti", addition: "", transitive: true, imperfective: true },
    ],
    "gazovati" => &[
        VerbDictionaryEntry { lemma: "gazovati", addition: "", transitive: true, imperfective: true },
    ],
    "generalizovati" => &[
        VerbDictionaryEntry { lemma: "generalizovati", addition: "", transitive: true, imperfective: true },
    ],
    "generovati" => &[
        VerbDictionaryEntry { lemma: "generovati", addition: "", transitive: true, imperfective: true },
    ],
    "gestikulovati" => &[
        VerbDictionaryEntry { lemma: "gestikulovati", addition: "", transitive: true, imperfective: true },
    ],
    "gladiti" => &[
        VerbDictionaryEntry { lemma: "gladiti", addition: "", transitive: true, imperfective: true },
    ],
    "globalizovati" => &[
        VerbDictionaryEntry { lemma: "globalizovati", addition: "", transitive: true, imperfective: true },
    ],
    "glodati" => &[
        VerbDictionaryEntry { lemma: "glodati", addition: "", transitive: true, imperfective: true },
    ],
    "gluhnųti" => &[
        VerbDictionaryEntry { lemma: "gluhnųti", addition: "", transitive: true, imperfective: true },
    ],
    "glupěti" => &[
        VerbDictionaryEntry { lemma: "glupěti", addition: "", transitive: true, imperfective: true },
    ],
    "glušiti" => &[
        VerbDictionaryEntry { lemma: "glušiti", addition: "", transitive: true, imperfective: true },
    ],
    "glådovati" => &[
        VerbDictionaryEntry { lemma: "glådovati", addition: "", transitive: true, imperfective: true },
    ],
    "glåsiti" => &[
        VerbDictionaryEntry { lemma: "glåsiti", addition: "", transitive: true, imperfective: true },
    ],
    "glåsovati" => &[
        VerbDictionaryEntry { lemma: "glåsovati", addition: "", transitive: true, imperfective: true },
    ],
    "ględati" => &[
        VerbDictionaryEntry { lemma: "ględati", addition: "", transitive: true, imperfective: true },
    ],
    "ględnųti" => &[
        VerbDictionaryEntry { lemma: "ględnųti", addition: "", transitive: true, imperfective: false },
    ],
    "ględěti" => &[
        VerbDictionaryEntry { lemma: "ględěti", addition: "(ględi)", transitive: true, imperfective: true },
    ],
    "gnati" => &[
        VerbDictionaryEntry { lemma: "gnati", addition: "(gone)", transitive: true, imperfective: true },
    ],
    "gnesti" => &[
        VerbDictionaryEntry { lemma: "gnesti", addition: "(gnete)", transitive: true, imperfective: true },
    ],
    "gniti" => &[
        VerbDictionaryEntry { lemma: "gniti", addition: "(gnije)", transitive: true, imperfective: true },
    ],
    "gnojiti" => &[
        VerbDictionaryEntry { lemma: "gnojiti", addition: "", transitive: true, imperfective: true },
    ],
    "gněvati" => &[
        VerbDictionaryEntry { lemma: "gněvati", addition: "", transitive: true, imperfective: true },
    ],
    "gnųti" => &[
        VerbDictionaryEntry { lemma: "gnųti", addition: "", transitive: true, imperfective: true },
    ],
    "godovati" => &[
        VerbDictionaryEntry { lemma: "godovati", addition: "", transitive: true, imperfective: true },
    ],
    "gojiti" => &[
        VerbDictionaryEntry { lemma: "gojiti", addition: "", transitive: true, imperfective: true },
    ],
    "goliti" => &[
        VerbDictionaryEntry { lemma: "goliti", addition: "", transitive: true, imperfective: true },
    ],
    "gorčiti" => &[
        VerbDictionaryEntry { lemma: "gorčiti", addition: "", transitive: true, imperfective: true },
    ],
    "gorěti" => &[
        VerbDictionaryEntry { lemma: "gorěti", addition: "(gori)", transitive: true, imperfective: true },
    ],
    "goršiti" => &[
        VerbDictionaryEntry { lemma: "goršiti", addition: "", transitive: true, imperfective: true },
    ],
    "gostiti" => &[
        VerbDictionaryEntry { lemma: "gostiti", addition: "", transitive: true, imperfective: true },
    ],
    "gotoviti" => &[
        VerbDictionaryEntry { lemma: "gotoviti", addition: "", transitive: true, imperfective: true },
    ],
    "govoriti" => &[
        VerbDictionaryEntry { lemma: "govoriti", addition: "", transitive: true, imperfective: true },
    ],
    "goŕknųti" => &[
        VerbDictionaryEntry { lemma: "goŕknųti", addition: "", transitive: true, imperfective: true },
    ],
    "grabiti" => &[
        VerbDictionaryEntry { lemma: "grabiti", addition: "", transitive: true, imperfective: true },
    ],
    "graničiti" => &[
        VerbDictionaryEntry { lemma: "graničiti", addition: "", transitive: true, imperfective: true },
    ],
    "gravirovati" => &[
        VerbDictionaryEntry { lemma: "gravirovati", addition: "", transitive: true, imperfective: true },
    ],
    "grditi" => &[
        VerbDictionaryEntry { lemma: "grditi", addition: "", transitive: true, imperfective: true },
    ],
    "grebti" => &[
        VerbDictionaryEntry { lemma: "grebti", addition: "", transitive: true, imperfective: true },
    ],
    "grnųti" => &[
        VerbDictionaryEntry { lemma: "grnųti", addition: "", transitive: true, imperfective: true },
    ],
    "grohtati" => &[
        VerbDictionaryEntry { lemma: "grohtati", addition: "", transitive: true, imperfective: true },
    ],
    "gromaditi" => &[
        VerbDictionaryEntry { lemma: "gromaditi", addition: "", transitive: true, imperfective: true },
    ],
    "groziti" => &[
        VerbDictionaryEntry { lemma: "groziti", addition: "", transitive: true, imperfective: true },
    ],
    "grupovati" => &[
        VerbDictionaryEntry { lemma: "grupovati", addition: "", transitive: true, imperfective: true },
    ],
    "gryzati" => &[
        VerbDictionaryEntry { lemma: "gryzati", addition: "", transitive: true, imperfective: true },
    ],
    "gryzti" => &[
        VerbDictionaryEntry { lemma: "gryzti", addition: "", transitive: true, imperfective: true },
    ],
    "grėměti" => &[
        VerbDictionaryEntry { lemma: "grėměti", addition: "(grėmi)", transitive: true, imperfective: true },
    ],
    "gręznųti" => &[
        VerbDictionaryEntry { lemma: "gręznųti", addition: "", transitive: true, imperfective: true },
    ],
    "grěti" => &[
        VerbDictionaryEntry { lemma: "grěti", addition: "(grěje)", transitive: true, imperfective: true },
    ],
    "grěšiti" => &[
        VerbDictionaryEntry { lemma: "grěšiti", addition: "", transitive: true, imperfective: true },
    ],
    "gubiti" => &[
        VerbDictionaryEntry { lemma: "gubiti", addition: "", transitive: true, imperfective: true },
    ],
    "gybati" => &[
        VerbDictionaryEntry { lemma: "gybati", addition: "", transitive: true, imperfective: true },
    ],
    "gybnųti" => &[
        VerbDictionaryEntry { lemma: "gybnųti", addition: "", transitive: true, imperfective: false },
    ],
    "gųstiti" => &[
        VerbDictionaryEntry { lemma: "gųstiti", addition: "", transitive: true, imperfective: true },
    ],
    "gųstěti" => &[
        VerbDictionaryEntry { lemma: "gųstěti", addition: "", transitive: true, imperfective: true },
    ],
    "gȯltati" => &[
        VerbDictionaryEntry { lemma: "gȯltati", addition: "", transitive: true, imperfective: true },
    ],
    "gȯltnųti" => &[
        VerbDictionaryEntry { lemma: "gȯltnųti", addition: "", transitive: true, imperfective: false },
    ],
    "halucinovati" => &[
        VerbDictionaryEntry { lemma: "halucinovati", addition: "", transitive: true, imperfective: true },
    ],
    "harakterizovati" => &[
        VerbDictionaryEntry { lemma: "harakterizovati", addition: "", transitive: true, imperfective: true },
    ],
    "harmonizovati" => &[
        VerbDictionaryEntry { lemma: "harmonizovati", addition: "", transitive: true, imperfective: true },
    ],
    "hipnotizovati" => &[
        VerbDictionaryEntry { lemma: "hipnotizovati", addition: "", transitive: true, imperfective: true },
    ],
    "hlipati" => &[
        VerbDictionaryEntry { lemma: "hlipati", addition: "", transitive: true, imperfective: true },
    ],
    "hlipnųti" => &[
        VerbDictionaryEntry { lemma: "hlipnųti", addition: "", transitive: true, imperfective: false },
    ],
    "hlåditi" => &[
        VerbDictionaryEntry { lemma: "hlåditi", addition: "", transitive: true, imperfective: true },
    ],
    "hlåděti" => &[
        VerbDictionaryEntry { lemma: "hlåděti", addition: "", transitive: true, imperfective: true },
    ],
    "hmuriti" => &[
        VerbDictionaryEntry { lemma: "hmuriti", addition: "", transitive: true, imperfective: true },
    ],
    "hoditi" => &[
        VerbDictionaryEntry { lemma: "hoditi", addition: "", transitive: true, imperfective: true },
    ],
    "homogenizovati" => &[
        VerbDictionaryEntry { lemma: "homogenizovati", addition: "", transitive: true, imperfective: true },
    ],
    "hotěti" => &[
        VerbDictionaryEntry { lemma: "hotěti", addition: "(hoće)", transitive: false, imperfective: true },
    ],
    "hovati" => &[
        VerbDictionaryEntry { lemma: "hovati", addition: "(hovaje)", transitive: true, imperfective: true },
    ],
    "hrapati" => &[
        VerbDictionaryEntry { lemma: "hrapati", addition: "", transitive: true, imperfective: true },
    ],
    "hrapnųti" => &[
        VerbDictionaryEntry { lemma: "hrapnųti", addition: "", transitive: true, imperfective: false },
    ],
    "hromati" => &[
        VerbDictionaryEntry { lemma: "hromati", addition: "", transitive: true, imperfective: true },
    ],
    "hroměti" => &[
        VerbDictionaryEntry { lemma: "hroměti", addition: "", transitive: true, imperfective: true },
    ],
    "hrupati" => &[
        VerbDictionaryEntry { lemma: "hrupati", addition: "", transitive: true, imperfective: true },
    ],
    "hrustati" => &[
        VerbDictionaryEntry { lemma: "hrustati", addition: "", transitive: true, imperfective: true },
    ],
    "hråniti" => &[
        VerbDictionaryEntry { lemma: "hråniti", addition: "", transitive: true, imperfective: true },
    ],
    "hrčati" => &[
        VerbDictionaryEntry { lemma: "hrčati", addition: "", transitive: true, imperfective: true },
    ],
    "htěti" => &[
        VerbDictionaryEntry { lemma: "htěti", addition: "(hće)", transitive: false, imperfective: true },
    ],
    "hudnųti" => &[
        VerbDictionaryEntry { lemma: "hudnųti", addition: "", transitive: true, imperfective: true },
    ],
    "hvaliti" => &[
        VerbDictionaryEntry { lemma: "hvaliti", addition: "", transitive: true, imperfective: true },
    ],
    "hvatati" => &[
        VerbDictionaryEntry { lemma: "hvatati", addition: "", transitive: true, imperfective: true },
    ],
    "hvorěti" => &[
        VerbDictionaryEntry { lemma: "hvorěti", addition: "", transitive: true, imperfective: true },
    ],
    "hvějati" => &[
        VerbDictionaryEntry { lemma: "hvějati", addition: "(hvěje)", transitive: true, imperfective: true },
    ],
    "hybiti" => &[
        VerbDictionaryEntry { lemma: "hybiti", addition: "", transitive: true, imperfective: true },
    ],
    "idealizovati" => &[
        VerbDictionaryEntry { lemma: "idealizovati", addition: "", transitive: true, imperfective: true },
    ],
    "identifikovati" => &[
        VerbDictionaryEntry { lemma: "identifikovati", addition: "", transitive: true, imperfective: true },
    ],
    "idti" => &[
        VerbDictionaryEntry { lemma: "idti", addition: "(ide; šėl)", transitive: true, imperfective: true },
    ],
    "ignorovati" => &[
        VerbDictionaryEntry { lemma: "ignorovati", addition: "", transitive: true, imperfective: true },
    ],
    "igrati" => &[
        VerbDictionaryEntry { lemma: "igrati", addition: "", transitive: true, imperfective: true },
    ],
    "ikati" => &[
        VerbDictionaryEntry { lemma: "ikati", addition: "", transitive: true, imperfective: true },
    ],
    "ilustrovati" => &[
        VerbDictionaryEntry { lemma: "ilustrovati", addition: "", transitive: true, imperfective: true },
    ],
    "imati" => &[
        VerbDictionaryEntry { lemma: "imati", addition: "(imaje)", transitive: false, imperfective: true },
        VerbDictionaryEntry { lemma: "imati", addition: "(imaje)", transitive: true, imperfective: true },
    ],
    "imenovati" => &[
        VerbDictionaryEntry { lemma: "imenovati", addition: "", transitive: true, imperfective: true },
    ],
    "imigrovati" => &[
        VerbDictionaryEntry { lemma: "imigrovati", addition: "", transitive: true, imperfective: true },
    ],
    "imitovati" => &[
        VerbDictionaryEntry { lemma: "imitovati", addition: "", transitive: true, imperfective: true },
    ],
    "implantovati" => &[
        VerbDictionaryEntry { lemma: "implantovati", addition: "", transitive: true, imperfective: true },
    ],
    "imponovati" => &[
        VerbDictionaryEntry { lemma: "imponovati", addition: "", transitive: true, imperfective: true },
    ],
    "importovati" => &[
        VerbDictionaryEntry { lemma: "importovati", addition: "", transitive: true, imperfective: true },
    ],
    "improvizovati" => &[
        VerbDictionaryEntry { lemma: "improvizovati", addition: "", transitive: true, imperfective: true },
    ],
    "iměti" => &[
        VerbDictionaryEntry { lemma: "iměti", addition: "(imaje)", transitive: false, imperfective: true },
        VerbDictionaryEntry { lemma: "iměti", addition: "(imaje)", transitive: true, imperfective: true },
    ],
    "industrializovati" => &[
        VerbDictionaryEntry { lemma: "industrializovati", addition: "", transitive: true, imperfective: true },
    ],
    "informovati" => &[
        VerbDictionaryEntry { lemma: "informovati", addition: "", transitive: true, imperfective: true },
    ],
    "inicializovati" => &[
        VerbDictionaryEntry { lemma: "inicializovati", addition: "", transitive: true, imperfective: true },
    ],
    "inspirovati" => &[
        VerbDictionaryEntry { lemma: "inspirovati", addition: "", transitive: true, imperfective: true },
    ],
    "instalovati" => &[
        VerbDictionaryEntry { lemma: "instalovati", addition: "", transitive: true, imperfective: true },
    ],
    "integrovati" => &[
        VerbDictionaryEntry { lemma: "integrovati", addition: "", transitive: true, imperfective: true },
    ],
    "interesovati" => &[
        VerbDictionaryEntry { lemma: "interesovati", addition: "", transitive: true, imperfective: true },
    ],
    "internacionalizovati" => &[
        VerbDictionaryEntry { lemma: "internacionalizovati", addition: "", transitive: true, imperfective: true },
    ],
    "interpretovati" => &[
        VerbDictionaryEntry { lemma: "interpretovati", addition: "", transitive: true, imperfective: true },
    ],
    "intrigovati" => &[
        VerbDictionaryEntry { lemma: "intrigovati", addition: "", transitive: true, imperfective: true },
    ],
    "investovati" => &[
        VerbDictionaryEntry { lemma: "investovati", addition: "", transitive: true, imperfective: true },
    ],
    "ironizovati" => &[
        VerbDictionaryEntry { lemma: "ironizovati", addition: "", transitive: true, imperfective: true },
    ],
    "iskati" => &[
        VerbDictionaryEntry { lemma: "iskati", addition: "(išče)", transitive: true, imperfective: true },
    ],
    "iskriti" => &[
        VerbDictionaryEntry { lemma: "iskriti", addition: "", transitive: true, imperfective: true },
    ],
    "istnovati" => &[
        VerbDictionaryEntry { lemma: "istnovati", addition: "", transitive: true, imperfective: true },
    ],
    "izbaviti" => &[
        VerbDictionaryEntry { lemma: "izbaviti", addition: "", transitive: true, imperfective: false },
        VerbDictionaryEntry { lemma: "izbaviti", addition: "(+2)", transitive: true, imperfective: false },
    ],
    "izbavjati" => &[
        VerbDictionaryEntry { lemma: "izbavjati", addition: "", transitive: true, imperfective: true },
        VerbDictionaryEntry { lemma: "izbavjati", addition: "(+2)", transitive: true, imperfective: true },
    ],
    "izbirati" => &[
        VerbDictionaryEntry { lemma: "izbirati", addition: "", transitive: true, imperfective: true },
    ],
    "izbiti" => &[
        VerbDictionaryEntry { lemma: "izbiti", addition: "(izbije)", transitive: true, imperfective: false },
    ],
    "izbivati" => &[
        VerbDictionaryEntry { lemma: "izbivati", addition: "", transitive: true, imperfective: true },
    ],
    "izbljuvati" => &[
        VerbDictionaryEntry { lemma: "izbljuvati", addition: "", transitive: true, imperfective: false },
    ],
    "izbombardovati" => &[
        VerbDictionaryEntry { lemma: "izbombardovati", addition: "", transitive: true, imperfective: false },
    ],
    "izbrati" => &[
        VerbDictionaryEntry { lemma: "izbrati", addition: "(izbere)", transitive: true, imperfective: false },
    ],
    "izbudovati" => &[
        VerbDictionaryEntry { lemma: "izbudovati", addition: "", transitive: true, imperfective: false },
    ],
    "izbuhati" => &[
        VerbDictionaryEntry { lemma: "izbuhati", addition: "", transitive: true, imperfective: true },
    ],
    "izbuhnųti" => &[
        VerbDictionaryEntry { lemma: "izbuhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "izběgati" => &[
        VerbDictionaryEntry { lemma: "izběgati", addition: "", transitive: true, imperfective: true },
        VerbDictionaryEntry { lemma: "izběgati", addition: "(+2)", transitive: true, imperfective: true },
    ],
    "izběgti" => &[
        VerbDictionaryEntry { lemma: "izběgti", addition: "(izběži) (+2)", transitive: true, imperfective: false },
        VerbDictionaryEntry { lemma: "izběgti", addition: "(izběži)", transitive: true, imperfective: false },
    ],
    "izcěliti" => &[
        VerbDictionaryEntry { lemma: "izcěliti", addition: "", transitive: true, imperfective: false },
    ],
    "izcěljati" => &[
        VerbDictionaryEntry { lemma: "izcěljati", addition: "", transitive: true, imperfective: true },
    ],
    "izdati" => &[
        VerbDictionaryEntry { lemma: "izdati", addition: "(izda)", transitive: true, imperfective: false },
    ],
    "izdavati" => &[
        VerbDictionaryEntry { lemma: "izdavati", addition: "", transitive: true, imperfective: true },
    ],
    "izdojiti" => &[
        VerbDictionaryEntry { lemma: "izdojiti", addition: "", transitive: true, imperfective: false },
    ],
    "izdumati" => &[
        VerbDictionaryEntry { lemma: "izdumati", addition: "", transitive: true, imperfective: false },
    ],
    "izdȯlbti" => &[
        VerbDictionaryEntry { lemma: "izdȯlbti", addition: "", transitive: true, imperfective: false },
    ],
    "izganjati" => &[
        VerbDictionaryEntry { lemma: "izganjati", addition: "", transitive: true, imperfective: true },
    ],
    "izgladiti" => &[
        VerbDictionaryEntry { lemma: "izgladiti", addition: "", transitive: true, imperfective: false },
    ],
    "izglašati" => &[
        VerbDictionaryEntry { lemma: "izglašati", addition: "", transitive: true, imperfective: true },
    ],
    "izglåsiti" => &[
        VerbDictionaryEntry { lemma: "izglåsiti", addition: "", transitive: true, imperfective: false },
    ],
    "izględati" => &[
        VerbDictionaryEntry { lemma: "izględati", addition: "", transitive: true, imperfective: true },
    ],
    "izględnųti" => &[
        VerbDictionaryEntry { lemma: "izględnųti", addition: "", transitive: true, imperfective: false },
    ],
    "izgnati" => &[
        VerbDictionaryEntry { lemma: "izgnati", addition: "(izgone)", transitive: true, imperfective: false },
    ],
    "izgniti" => &[
        VerbDictionaryEntry { lemma: "izgniti", addition: "(izgnije)", transitive: true, imperfective: false },
    ],
    "izgojiti" => &[
        VerbDictionaryEntry { lemma: "izgojiti", addition: "", transitive: true, imperfective: false },
    ],
    "izgorěti" => &[
        VerbDictionaryEntry { lemma: "izgorěti", addition: "", transitive: true, imperfective: false },
    ],
    "izgovarjati" => &[
        VerbDictionaryEntry { lemma: "izgovarjati", addition: "", transitive: true, imperfective: true },
    ],
    "izgovoriti" => &[
        VerbDictionaryEntry { lemma: "izgovoriti", addition: "", transitive: true, imperfective: false },
    ],
    "izgubiti" => &[
        VerbDictionaryEntry { lemma: "izgubiti", addition: "", transitive: true, imperfective: false },
    ],
    "izgynųti" => &[
        VerbDictionaryEntry { lemma: "izgynųti", addition: "", transitive: true, imperfective: false },
    ],
    "izhoditi" => &[
        VerbDictionaryEntry { lemma: "izhoditi", addition: "", transitive: true, imperfective: true },
    ],
    "izigrati" => &[
        VerbDictionaryEntry { lemma: "izigrati", addition: "", transitive: true, imperfective: false },
    ],
    "izigryvati" => &[
        VerbDictionaryEntry { lemma: "izigryvati", addition: "", transitive: true, imperfective: true },
    ],
    "izimati" => &[
        VerbDictionaryEntry { lemma: "izimati", addition: "", transitive: true, imperfective: true },
    ],
    "iziskati" => &[
        VerbDictionaryEntry { lemma: "iziskati", addition: "(izišče)", transitive: true, imperfective: false },
    ],
    "iziskyvati" => &[
        VerbDictionaryEntry { lemma: "iziskyvati", addition: "", transitive: true, imperfective: true },
    ],
    "izjasniti" => &[
        VerbDictionaryEntry { lemma: "izjasniti", addition: "", transitive: true, imperfective: false },
    ],
    "izjasnjati" => &[
        VerbDictionaryEntry { lemma: "izjasnjati", addition: "", transitive: true, imperfective: true },
    ],
    "izjaviti" => &[
        VerbDictionaryEntry { lemma: "izjaviti", addition: "", transitive: true, imperfective: false },
    ],
    "izjavjati" => &[
        VerbDictionaryEntry { lemma: "izjavjati", addition: "", transitive: true, imperfective: true },
    ],
    "izjehati" => &[
        VerbDictionaryEntry { lemma: "izjehati", addition: "(izjede)", transitive: true, imperfective: false },
    ],
    "izježđati" => &[
        VerbDictionaryEntry { lemma: "izježđati", addition: "", transitive: true, imperfective: true },
    ],
    "izjęti" => &[
        VerbDictionaryEntry { lemma: "izjęti", addition: "(izȯjme)", transitive: true, imperfective: false },
    ],
    "izkalkulovati" => &[
        VerbDictionaryEntry { lemma: "izkalkulovati", addition: "", transitive: true, imperfective: false },
    ],
    "izkladati" => &[
        VerbDictionaryEntry { lemma: "izkladati", addition: "", transitive: true, imperfective: true },
    ],
    "izključati" => &[
        VerbDictionaryEntry { lemma: "izključati", addition: "", transitive: true, imperfective: true },
    ],
    "izključiti" => &[
        VerbDictionaryEntry { lemma: "izključiti", addition: "", transitive: true, imperfective: false },
    ],
    "izkopati" => &[
        VerbDictionaryEntry { lemma: "izkopati", addition: "", transitive: true, imperfective: false },
    ],
    "izkopyvati" => &[
        VerbDictionaryEntry { lemma: "izkopyvati", addition: "", transitive: true, imperfective: true },
    ],
    "izkoreniti" => &[
        VerbDictionaryEntry { lemma: "izkoreniti", addition: "", transitive: true, imperfective: false },
    ],
    "izkorenjati" => &[
        VerbDictionaryEntry { lemma: "izkorenjati", addition: "", transitive: true, imperfective: true },
    ],
    "izkoristati" => &[
        VerbDictionaryEntry { lemma: "izkoristati", addition: "", transitive: true, imperfective: false },
    ],
    "izkoristiti" => &[
        VerbDictionaryEntry { lemma: "izkoristiti", addition: "", transitive: true, imperfective: false },
    ],
    "izkoristyvati" => &[
        VerbDictionaryEntry { lemma: "izkoristyvati", addition: "", transitive: true, imperfective: true },
    ],
    "izkositi" => &[
        VerbDictionaryEntry { lemma: "izkositi", addition: "", transitive: true, imperfective: false },
    ],
    "izkovati" => &[
        VerbDictionaryEntry { lemma: "izkovati", addition: "", transitive: true, imperfective: false },
    ],
    "izkovyvati" => &[
        VerbDictionaryEntry { lemma: "izkovyvati", addition: "", transitive: true, imperfective: true },
    ],
    "izkrikati" => &[
        VerbDictionaryEntry { lemma: "izkrikati", addition: "", transitive: true, imperfective: true },
    ],
    "izkriknųti" => &[
        VerbDictionaryEntry { lemma: "izkriknųti", addition: "", transitive: true, imperfective: false },
    ],
    "izkrviti" => &[
        VerbDictionaryEntry { lemma: "izkrviti", addition: "", transitive: true, imperfective: false },
    ],
    "izkupiti" => &[
        VerbDictionaryEntry { lemma: "izkupiti", addition: "", transitive: true, imperfective: false },
    ],
    "izkusiti" => &[
        VerbDictionaryEntry { lemma: "izkusiti", addition: "", transitive: true, imperfective: false },
    ],
    "izkušati" => &[
        VerbDictionaryEntry { lemma: "izkušati", addition: "", transitive: true, imperfective: true },
    ],
    "izkydati" => &[
        VerbDictionaryEntry { lemma: "izkydati", addition: "", transitive: true, imperfective: true },
    ],
    "izkydnųti" => &[
        VerbDictionaryEntry { lemma: "izkydnųti", addition: "", transitive: true, imperfective: false },
    ],
    "izkųpati" => &[
        VerbDictionaryEntry { lemma: "izkųpati", addition: "", transitive: true, imperfective: false },
    ],
    "izlagati" => &[
        VerbDictionaryEntry { lemma: "izlagati", addition: "", transitive: true, imperfective: true },
    ],
    "izliti" => &[
        VerbDictionaryEntry { lemma: "izliti", addition: "(izlije)", transitive: true, imperfective: false },
    ],
    "izlivati" => &[
        VerbDictionaryEntry { lemma: "izlivati", addition: "(izlije)", transitive: true, imperfective: true },
    ],
    "izlomiti" => &[
        VerbDictionaryEntry { lemma: "izlomiti", addition: "", transitive: true, imperfective: false },
    ],
    "izloviti" => &[
        VerbDictionaryEntry { lemma: "izloviti", addition: "", transitive: true, imperfective: false },
    ],
    "izložiti" => &[
        VerbDictionaryEntry { lemma: "izložiti", addition: "", transitive: true, imperfective: false },
    ],
    "izlęgti" => &[
        VerbDictionaryEntry { lemma: "izlęgti", addition: "", transitive: true, imperfective: false },
    ],
    "izlězati" => &[
        VerbDictionaryEntry { lemma: "izlězati", addition: "", transitive: true, imperfective: true },
    ],
    "izlězti" => &[
        VerbDictionaryEntry { lemma: "izlězti", addition: "", transitive: true, imperfective: false },
    ],
    "izlěčiti" => &[
        VerbDictionaryEntry { lemma: "izlěčiti", addition: "", transitive: true, imperfective: false },
    ],
    "izmagati" => &[
        VerbDictionaryEntry { lemma: "izmagati", addition: "", transitive: true, imperfective: true },
    ],
    "izmamiti" => &[
        VerbDictionaryEntry { lemma: "izmamiti", addition: "", transitive: true, imperfective: false },
    ],
    "izmamjati" => &[
        VerbDictionaryEntry { lemma: "izmamjati", addition: "", transitive: true, imperfective: true },
    ],
    "izmesti" => &[
        VerbDictionaryEntry { lemma: "izmesti", addition: "(izmete)", transitive: true, imperfective: false },
    ],
    "izmirati" => &[
        VerbDictionaryEntry { lemma: "izmirati", addition: "", transitive: true, imperfective: true },
    ],
    "izmodelovati" => &[
        VerbDictionaryEntry { lemma: "izmodelovati", addition: "", transitive: true, imperfective: false },
    ],
    "izmogti" => &[
        VerbDictionaryEntry { lemma: "izmogti", addition: "", transitive: true, imperfective: false },
    ],
    "izmoriti" => &[
        VerbDictionaryEntry { lemma: "izmoriti", addition: "", transitive: true, imperfective: false },
    ],
    "izmočiti" => &[
        VerbDictionaryEntry { lemma: "izmočiti", addition: "", transitive: true, imperfective: false },
    ],
    "izmrěti" => &[
        VerbDictionaryEntry { lemma: "izmrěti", addition: "", transitive: true, imperfective: false },
    ],
    "izmysliti" => &[
        VerbDictionaryEntry { lemma: "izmysliti", addition: "", transitive: true, imperfective: false },
    ],
    "izmysljati" => &[
        VerbDictionaryEntry { lemma: "izmysljati", addition: "", transitive: true, imperfective: true },
    ],
    "izměniti" => &[
        VerbDictionaryEntry { lemma: "izměniti", addition: "", transitive: true, imperfective: false },
    ],
    "izměnjati" => &[
        VerbDictionaryEntry { lemma: "izměnjati", addition: "", transitive: true, imperfective: true },
    ],
    "izměriti" => &[
        VerbDictionaryEntry { lemma: "izměriti", addition: "", transitive: true, imperfective: false },
    ],
    "izměstiti" => &[
        VerbDictionaryEntry { lemma: "izměstiti", addition: "", transitive: true, imperfective: false },
    ],
    "izmětati" => &[
        VerbDictionaryEntry { lemma: "izmětati", addition: "", transitive: true, imperfective: true },
    ],
    "izměšćati" => &[
        VerbDictionaryEntry { lemma: "izměšćati", addition: "", transitive: true, imperfective: true },
    ],
    "iznahoditi" => &[
        VerbDictionaryEntry { lemma: "iznahoditi", addition: "", transitive: true, imperfective: true },
    ],
    "iznajdti" => &[
        VerbDictionaryEntry { lemma: "iznajdti", addition: "(iznajde; iznašėl)", transitive: true, imperfective: false },
    ],
    "iznajmati" => &[
        VerbDictionaryEntry { lemma: "iznajmati", addition: "", transitive: true, imperfective: true },
    ],
    "iznajęti" => &[
        VerbDictionaryEntry { lemma: "iznajęti", addition: "(iznajme)", transitive: true, imperfective: false },
    ],
    "iznasilovati" => &[
        VerbDictionaryEntry { lemma: "iznasilovati", addition: "", transitive: true, imperfective: false },
    ],
    "iznemogti" => &[
        VerbDictionaryEntry { lemma: "iznemogti", addition: "", transitive: true, imperfective: false },
    ],
    "iznesti" => &[
        VerbDictionaryEntry { lemma: "iznesti", addition: "", transitive: true, imperfective: false },
    ],
    "izniknųti" => &[
        VerbDictionaryEntry { lemma: "izniknųti", addition: "", transitive: true, imperfective: false },
    ],
    "izniščiti" => &[
        VerbDictionaryEntry { lemma: "izniščiti", addition: "", transitive: true, imperfective: false },
    ],
    "iznositi" => &[
        VerbDictionaryEntry { lemma: "iznositi", addition: "", transitive: true, imperfective: true },
    ],
    "iznuditi" => &[
        VerbDictionaryEntry { lemma: "iznuditi", addition: "", transitive: true, imperfective: false },
    ],
    "iznuriti" => &[
        VerbDictionaryEntry { lemma: "iznuriti", addition: "", transitive: true, imperfective: false },
    ],
    "iznurjati" => &[
        VerbDictionaryEntry { lemma: "iznurjati", addition: "", transitive: true, imperfective: true },
    ],
    "iznuđati" => &[
        VerbDictionaryEntry { lemma: "iznuđati", addition: "", transitive: true, imperfective: true },
    ],
    "izobličiti" => &[
        VerbDictionaryEntry { lemma: "izobličiti", addition: "", transitive: true, imperfective: false },
    ],
    "izobraziti" => &[
        VerbDictionaryEntry { lemma: "izobraziti", addition: "", transitive: true, imperfective: false },
    ],
    "izobražati" => &[
        VerbDictionaryEntry { lemma: "izobražati", addition: "", transitive: true, imperfective: true },
    ],
    "izolovati" => &[
        VerbDictionaryEntry { lemma: "izolovati", addition: "", transitive: true, imperfective: true },
    ],
    "izopačati" => &[
        VerbDictionaryEntry { lemma: "izopačati", addition: "", transitive: true, imperfective: true },
    ],
    "izopačiti" => &[
        VerbDictionaryEntry { lemma: "izopačiti", addition: "", transitive: true, imperfective: false },
    ],
    "izorati" => &[
        VerbDictionaryEntry { lemma: "izorati", addition: "(izoŕe)", transitive: true, imperfective: false },
    ],
    "izpadati" => &[
        VerbDictionaryEntry { lemma: "izpadati", addition: "", transitive: true, imperfective: true },
    ],
    "izpasti" => &[
        VerbDictionaryEntry { lemma: "izpasti", addition: "(izpade)", transitive: true, imperfective: false },
    ],
    "izpekti" => &[
        VerbDictionaryEntry { lemma: "izpekti", addition: "", transitive: true, imperfective: false },
    ],
    "izpepeliti" => &[
        VerbDictionaryEntry { lemma: "izpepeliti", addition: "", transitive: true, imperfective: false },
    ],
    "izphati" => &[
        VerbDictionaryEntry { lemma: "izphati", addition: "", transitive: true, imperfective: false },
    ],
    "izpihati" => &[
        VerbDictionaryEntry { lemma: "izpihati", addition: "", transitive: true, imperfective: true },
    ],
    "izpiti" => &[
        VerbDictionaryEntry { lemma: "izpiti", addition: "(izpije)", transitive: true, imperfective: false },
    ],
    "izplatiti" => &[
        VerbDictionaryEntry { lemma: "izplatiti", addition: "", transitive: true, imperfective: false },
    ],
    "izplaćati" => &[
        VerbDictionaryEntry { lemma: "izplaćati", addition: "", transitive: true, imperfective: true },
    ],
    "izpljunųti" => &[
        VerbDictionaryEntry { lemma: "izpljunųti", addition: "", transitive: true, imperfective: false },
    ],
    "izpljuvati" => &[
        VerbDictionaryEntry { lemma: "izpljuvati", addition: "", transitive: true, imperfective: true },
    ],
    "izpluti" => &[
        VerbDictionaryEntry { lemma: "izpluti", addition: "(izplove)", transitive: true, imperfective: false },
    ],
    "izplyvati" => &[
        VerbDictionaryEntry { lemma: "izplyvati", addition: "", transitive: true, imperfective: true },
    ],
    "izplyvti" => &[
        VerbDictionaryEntry { lemma: "izplyvti", addition: "", transitive: true, imperfective: false },
    ],
    "izplåšiti" => &[
        VerbDictionaryEntry { lemma: "izplåšiti", addition: "", transitive: true, imperfective: false },
    ],
    "izplěti" => &[
        VerbDictionaryEntry { lemma: "izplěti", addition: "(izplěje/izplěve)", transitive: true, imperfective: false },
    ],
    "izpovědati" => &[
        VerbDictionaryEntry { lemma: "izpovědati", addition: "", transitive: true, imperfective: true },
    ],
    "izpověděti" => &[
        VerbDictionaryEntry { lemma: "izpověděti", addition: "(izpově)", transitive: true, imperfective: false },
    ],
    "izprati" => &[
        VerbDictionaryEntry { lemma: "izprati", addition: "(izpere)", transitive: true, imperfective: false },
    ],
    "izpraviti" => &[
        VerbDictionaryEntry { lemma: "izpraviti", addition: "", transitive: true, imperfective: false },
    ],
    "izpravjati" => &[
        VerbDictionaryEntry { lemma: "izpravjati", addition: "", transitive: true, imperfective: true },
    ],
    "izprašati" => &[
        VerbDictionaryEntry { lemma: "izprašati", addition: "", transitive: true, imperfective: true },
    ],
    "izprobovati" => &[
        VerbDictionaryEntry { lemma: "izprobovati", addition: "", transitive: true, imperfective: false },
    ],
    "izprobovyvati" => &[
        VerbDictionaryEntry { lemma: "izprobovyvati", addition: "", transitive: true, imperfective: true },
    ],
    "izprositi" => &[
        VerbDictionaryEntry { lemma: "izprositi", addition: "", transitive: true, imperfective: false },
    ],
    "izpråzdniti" => &[
        VerbDictionaryEntry { lemma: "izpråzdniti", addition: "", transitive: true, imperfective: false },
    ],
    "izpråzdnjati" => &[
        VerbDictionaryEntry { lemma: "izpråzdnjati", addition: "", transitive: true, imperfective: true },
    ],
    "izpręgati" => &[
        VerbDictionaryEntry { lemma: "izpręgati", addition: "", transitive: true, imperfective: true },
    ],
    "izpręgti" => &[
        VerbDictionaryEntry { lemma: "izpręgti", addition: "", transitive: true, imperfective: false },
    ],
    "izpustiti" => &[
        VerbDictionaryEntry { lemma: "izpustiti", addition: "", transitive: true, imperfective: false },
    ],
    "izpušćati" => &[
        VerbDictionaryEntry { lemma: "izpušćati", addition: "", transitive: true, imperfective: true },
    ],
    "izpytati" => &[
        VerbDictionaryEntry { lemma: "izpytati", addition: "", transitive: true, imperfective: false },
    ],
    "izpytyvati" => &[
        VerbDictionaryEntry { lemma: "izpytyvati", addition: "", transitive: true, imperfective: true },
    ],
    "izpųditi" => &[
        VerbDictionaryEntry { lemma: "izpųditi", addition: "", transitive: true, imperfective: false },
    ],
    "izpȯlniti" => &[
        VerbDictionaryEntry { lemma: "izpȯlniti", addition: "", transitive: true, imperfective: false },
    ],
    "izpȯlnjati" => &[
        VerbDictionaryEntry { lemma: "izpȯlnjati", addition: "", transitive: true, imperfective: true },
    ],
    "izpȯlzati" => &[
        VerbDictionaryEntry { lemma: "izpȯlzati", addition: "", transitive: true, imperfective: true },
    ],
    "izpȯlzti" => &[
        VerbDictionaryEntry { lemma: "izpȯlzti", addition: "", transitive: true, imperfective: false },
    ],
    "izravnjati" => &[
        VerbDictionaryEntry { lemma: "izravnjati", addition: "", transitive: true, imperfective: true },
    ],
    "izraziti" => &[
        VerbDictionaryEntry { lemma: "izraziti", addition: "", transitive: true, imperfective: false },
    ],
    "izražati" => &[
        VerbDictionaryEntry { lemma: "izražati", addition: "", transitive: true, imperfective: true },
    ],
    "izrvati" => &[
        VerbDictionaryEntry { lemma: "izrvati", addition: "(izrve)", transitive: true, imperfective: false },
    ],
    "izryvati" => &[
        VerbDictionaryEntry { lemma: "izryvati", addition: "", transitive: true, imperfective: true },
    ],
    "izråbiti" => &[
        VerbDictionaryEntry { lemma: "izråbiti", addition: "", transitive: true, imperfective: false },
    ],
    "izråbotati" => &[
        VerbDictionaryEntry { lemma: "izråbotati", addition: "", transitive: true, imperfective: false },
    ],
    "izråbotyvati" => &[
        VerbDictionaryEntry { lemma: "izråbotyvati", addition: "", transitive: true, imperfective: true },
    ],
    "izråsti" => &[
        VerbDictionaryEntry { lemma: "izråsti", addition: "(izråste)", transitive: true, imperfective: false },
    ],
    "izråvniti" => &[
        VerbDictionaryEntry { lemma: "izråvniti", addition: "", transitive: true, imperfective: false },
    ],
    "izrěkati" => &[
        VerbDictionaryEntry { lemma: "izrěkati", addition: "", transitive: true, imperfective: true },
    ],
    "izrěkti" => &[
        VerbDictionaryEntry { lemma: "izrěkti", addition: "", transitive: true, imperfective: false },
    ],
    "izrězati" => &[
        VerbDictionaryEntry { lemma: "izrězati", addition: "(izrěže)", transitive: true, imperfective: false },
    ],
    "izrězyvati" => &[
        VerbDictionaryEntry { lemma: "izrězyvati", addition: "", transitive: true, imperfective: true },
    ],
    "izrųbati" => &[
        VerbDictionaryEntry { lemma: "izrųbati", addition: "", transitive: true, imperfective: false },
    ],
    "izrųbyvati" => &[
        VerbDictionaryEntry { lemma: "izrųbyvati", addition: "", transitive: true, imperfective: true },
    ],
    "izseliti" => &[
        VerbDictionaryEntry { lemma: "izseliti", addition: "", transitive: true, imperfective: false },
    ],
    "izseljati" => &[
        VerbDictionaryEntry { lemma: "izseljati", addition: "", transitive: true, imperfective: true },
    ],
    "izslati" => &[
        VerbDictionaryEntry { lemma: "izslati", addition: "(izšlje)", transitive: true, imperfective: false },
    ],
    "izslavjati" => &[
        VerbDictionaryEntry { lemma: "izslavjati", addition: "", transitive: true, imperfective: true },
    ],
    "izsloviti" => &[
        VerbDictionaryEntry { lemma: "izsloviti", addition: "", transitive: true, imperfective: false },
    ],
    "izslušati" => &[
        VerbDictionaryEntry { lemma: "izslušati", addition: "", transitive: true, imperfective: false },
    ],
    "izslušivati" => &[
        VerbDictionaryEntry { lemma: "izslušivati", addition: "", transitive: true, imperfective: true },
    ],
    "izslědovati" => &[
        VerbDictionaryEntry { lemma: "izslědovati", addition: "", transitive: true, imperfective: true },
    ],
    "izsmějati" => &[
        VerbDictionaryEntry { lemma: "izsmějati", addition: "(izsměje)", transitive: true, imperfective: false },
    ],
    "izsmějivati" => &[
        VerbDictionaryEntry { lemma: "izsmějivati", addition: "", transitive: true, imperfective: true },
    ],
    "izstaviti" => &[
        VerbDictionaryEntry { lemma: "izstaviti", addition: "", transitive: true, imperfective: false },
    ],
    "izstavjati" => &[
        VerbDictionaryEntry { lemma: "izstavjati", addition: "", transitive: true, imperfective: true },
    ],
    "izstrěliti" => &[
        VerbDictionaryEntry { lemma: "izstrěliti", addition: "", transitive: true, imperfective: false },
    ],
    "izstųpati" => &[
        VerbDictionaryEntry { lemma: "izstųpati", addition: "", transitive: true, imperfective: true },
    ],
    "izstųpiti" => &[
        VerbDictionaryEntry { lemma: "izstųpiti", addition: "", transitive: true, imperfective: false },
    ],
    "izsunųti" => &[
        VerbDictionaryEntry { lemma: "izsunųti", addition: "", transitive: true, imperfective: false },
    ],
    "izsuvati" => &[
        VerbDictionaryEntry { lemma: "izsuvati", addition: "", transitive: true, imperfective: true },
    ],
    "izsušati" => &[
        VerbDictionaryEntry { lemma: "izsušati", addition: "", transitive: true, imperfective: true },
    ],
    "izsušiti" => &[
        VerbDictionaryEntry { lemma: "izsušiti", addition: "", transitive: true, imperfective: false },
    ],
    "izsyhati" => &[
        VerbDictionaryEntry { lemma: "izsyhati", addition: "", transitive: true, imperfective: true },
    ],
    "izsylati" => &[
        VerbDictionaryEntry { lemma: "izsylati", addition: "", transitive: true, imperfective: true },
    ],
    "izsypati" => &[
        VerbDictionaryEntry { lemma: "izsypati", addition: "", transitive: true, imperfective: false },
    ],
    "izsysati" => &[
        VerbDictionaryEntry { lemma: "izsysati", addition: "", transitive: true, imperfective: true },
    ],
    "izsěkati" => &[
        VerbDictionaryEntry { lemma: "izsěkati", addition: "", transitive: true, imperfective: true },
    ],
    "izsěkti" => &[
        VerbDictionaryEntry { lemma: "izsěkti", addition: "", transitive: true, imperfective: false },
    ],
    "izsȯhnųti" => &[
        VerbDictionaryEntry { lemma: "izsȯhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "izsȯsati" => &[
        VerbDictionaryEntry { lemma: "izsȯsati", addition: "", transitive: true, imperfective: false },
    ],
    "iztekti" => &[
        VerbDictionaryEntry { lemma: "iztekti", addition: "", transitive: true, imperfective: false },
    ],
    "iztirati" => &[
        VerbDictionaryEntry { lemma: "iztirati", addition: "", transitive: true, imperfective: true },
    ],
    "iztkati" => &[
        VerbDictionaryEntry { lemma: "iztkati", addition: "", transitive: true, imperfective: false },
    ],
    "iztratiti" => &[
        VerbDictionaryEntry { lemma: "iztratiti", addition: "", transitive: true, imperfective: false },
    ],
    "iztrgati" => &[
        VerbDictionaryEntry { lemma: "iztrgati", addition: "", transitive: true, imperfective: true },
    ],
    "iztrgnųti" => &[
        VerbDictionaryEntry { lemma: "iztrgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "iztręsati" => &[
        VerbDictionaryEntry { lemma: "iztręsati", addition: "", transitive: true, imperfective: true },
    ],
    "iztręsti" => &[
        VerbDictionaryEntry { lemma: "iztręsti", addition: "", transitive: true, imperfective: false },
    ],
    "iztrěti" => &[
        VerbDictionaryEntry { lemma: "iztrěti", addition: "(iztre)", transitive: true, imperfective: false },
    ],
    "iztvarjati" => &[
        VerbDictionaryEntry { lemma: "iztvarjati", addition: "", transitive: true, imperfective: true },
    ],
    "iztvoriti" => &[
        VerbDictionaryEntry { lemma: "iztvoriti", addition: "", transitive: true, imperfective: false },
    ],
    "iztęgati" => &[
        VerbDictionaryEntry { lemma: "iztęgati", addition: "", transitive: true, imperfective: true },
    ],
    "iztęgnųti" => &[
        VerbDictionaryEntry { lemma: "iztęgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "iztěkati" => &[
        VerbDictionaryEntry { lemma: "iztěkati", addition: "", transitive: true, imperfective: true },
    ],
    "iztŕpěti" => &[
        VerbDictionaryEntry { lemma: "iztŕpěti", addition: "(iztŕpi)", transitive: true, imperfective: false },
    ],
    "iztŕti" => &[
        VerbDictionaryEntry { lemma: "iztŕti", addition: "(iztre)", transitive: true, imperfective: false },
    ],
    "iztȯlkti" => &[
        VerbDictionaryEntry { lemma: "iztȯlkti", addition: "", transitive: true, imperfective: false },
    ],
    "izumirati" => &[
        VerbDictionaryEntry { lemma: "izumirati", addition: "", transitive: true, imperfective: true },
    ],
    "izumrěti" => &[
        VerbDictionaryEntry { lemma: "izumrěti", addition: "(izumre)", transitive: true, imperfective: false },
    ],
    "izumŕti" => &[
        VerbDictionaryEntry { lemma: "izumŕti", addition: "(izumre)", transitive: true, imperfective: false },
    ],
    "izučati" => &[
        VerbDictionaryEntry { lemma: "izučati", addition: "", transitive: true, imperfective: true },
    ],
    "izučiti" => &[
        VerbDictionaryEntry { lemma: "izučiti", addition: "", transitive: true, imperfective: false },
    ],
    "izvajati" => &[
        VerbDictionaryEntry { lemma: "izvajati", addition: "", transitive: true, imperfective: false },
    ],
    "izvaljnjati" => &[
        VerbDictionaryEntry { lemma: "izvaljnjati", addition: "", transitive: true, imperfective: true },
    ],
    "izvesti" => &[
        VerbDictionaryEntry { lemma: "izvesti", addition: "(izvede)", transitive: true, imperfective: false },
    ],
    "izvezti" => &[
        VerbDictionaryEntry { lemma: "izvezti", addition: "", transitive: true, imperfective: false },
    ],
    "izvihnųti" => &[
        VerbDictionaryEntry { lemma: "izvihnųti", addition: "", transitive: true, imperfective: false },
    ],
    "izviniti" => &[
        VerbDictionaryEntry { lemma: "izviniti", addition: "", transitive: true, imperfective: false },
    ],
    "izvinjati" => &[
        VerbDictionaryEntry { lemma: "izvinjati", addition: "", transitive: true, imperfective: true },
    ],
    "izvlastniti" => &[
        VerbDictionaryEntry { lemma: "izvlastniti", addition: "", transitive: true, imperfective: false },
    ],
    "izvlastnjati" => &[
        VerbDictionaryEntry { lemma: "izvlastnjati", addition: "", transitive: true, imperfective: true },
    ],
    "izvlěkati" => &[
        VerbDictionaryEntry { lemma: "izvlěkati", addition: "", transitive: true, imperfective: true },
    ],
    "izvlěkti" => &[
        VerbDictionaryEntry { lemma: "izvlěkti", addition: "", transitive: true, imperfective: false },
    ],
    "izvoditi" => &[
        VerbDictionaryEntry { lemma: "izvoditi", addition: "", transitive: true, imperfective: true },
    ],
    "izvoliti" => &[
        VerbDictionaryEntry { lemma: "izvoliti", addition: "", transitive: true, imperfective: false },
    ],
    "izvoljniti" => &[
        VerbDictionaryEntry { lemma: "izvoljniti", addition: "", transitive: true, imperfective: false },
    ],
    "izvoziti" => &[
        VerbDictionaryEntry { lemma: "izvoziti", addition: "", transitive: true, imperfective: true },
    ],
    "izvraćati" => &[
        VerbDictionaryEntry { lemma: "izvraćati", addition: "", transitive: true, imperfective: true },
    ],
    "izvråtiti" => &[
        VerbDictionaryEntry { lemma: "izvråtiti", addition: "", transitive: true, imperfective: false },
    ],
    "izvěstiti" => &[
        VerbDictionaryEntry { lemma: "izvěstiti", addition: "", transitive: true, imperfective: false },
    ],
    "izvěšćati" => &[
        VerbDictionaryEntry { lemma: "izvěšćati", addition: "", transitive: true, imperfective: true },
    ],
    "izznačati" => &[
        VerbDictionaryEntry { lemma: "izznačati", addition: "", transitive: true, imperfective: true },
    ],
    "izznačiti" => &[
        VerbDictionaryEntry { lemma: "izznačiti", addition: "", transitive: true, imperfective: false },
    ],
    "izzvati" => &[
        VerbDictionaryEntry { lemma: "izzvati", addition: "(izzȯve)", transitive: true, imperfective: false },
    ],
    "izzyvati" => &[
        VerbDictionaryEntry { lemma: "izzyvati", addition: "", transitive: true, imperfective: true },
    ],
    "izčezati" => &[
        VerbDictionaryEntry { lemma: "izčezati", addition: "", transitive: true, imperfective: true },
    ],
    "izčeznųti" => &[
        VerbDictionaryEntry { lemma: "izčeznųti", addition: "", transitive: true, imperfective: false },
    ],
    "izčisliti" => &[
        VerbDictionaryEntry { lemma: "izčisliti", addition: "", transitive: true, imperfective: false },
    ],
    "izčisljati" => &[
        VerbDictionaryEntry { lemma: "izčisljati", addition: "", transitive: true, imperfective: true },
    ],
    "izčistiti" => &[
        VerbDictionaryEntry { lemma: "izčistiti", addition: "", transitive: true, imperfective: false },
    ],
    "izčrkati" => &[
        VerbDictionaryEntry { lemma: "izčrkati", addition: "", transitive: true, imperfective: true },
    ],
    "izčrknųti" => &[
        VerbDictionaryEntry { lemma: "izčrknųti", addition: "", transitive: true, imperfective: false },
    ],
    "izčrpati" => &[
        VerbDictionaryEntry { lemma: "izčrpati", addition: "(izčrpe)", transitive: true, imperfective: false },
    ],
    "izčrpyvati" => &[
        VerbDictionaryEntry { lemma: "izčrpyvati", addition: "", transitive: true, imperfective: true },
    ],
    "izšiti" => &[
        VerbDictionaryEntry { lemma: "izšiti", addition: "(izšije)", transitive: true, imperfective: false },
    ],
    "izšivati" => &[
        VerbDictionaryEntry { lemma: "izšivati", addition: "", transitive: true, imperfective: true },
    ],
    "izškoliti" => &[
        VerbDictionaryEntry { lemma: "izškoliti", addition: "", transitive: true, imperfective: false },
    ],
    "izžęti" => &[
        VerbDictionaryEntry { lemma: "izžęti", addition: "(izžne)", transitive: true, imperfective: false },
    ],
    "izȯjdti" => &[
        VerbDictionaryEntry { lemma: "izȯjdti", addition: "(izȯjde; izšėl)", transitive: true, imperfective: false },
    ],
    "jalověti" => &[
        VerbDictionaryEntry { lemma: "jalověti", addition: "", transitive: true, imperfective: true },
    ],
    "jasněti" => &[
        VerbDictionaryEntry { lemma: "jasněti", addition: "", transitive: true, imperfective: true },
    ],
    "je" => &[
        VerbDictionaryEntry { lemma: "je", addition: "", transitive: false, imperfective: true },
    ],
    "jebati" => &[
        VerbDictionaryEntry { lemma: "jebati", addition: "(jebe)", transitive: true, imperfective: true },
    ],
    "jedati" => &[
        VerbDictionaryEntry { lemma: "jedati", addition: "", transitive: true, imperfective: true },
    ],
    "jehati" => &[
        VerbDictionaryEntry { lemma: "jehati", addition: "(jede)", transitive: true, imperfective: true },
    ],
    "jest" => &[
        VerbDictionaryEntry { lemma: "jest", addition: "", transitive: false, imperfective: true },
    ],
    "jesti" => &[
        VerbDictionaryEntry { lemma: "jesti", addition: "(je)", transitive: true, imperfective: true },
    ],
    "jestvovati" => &[
        VerbDictionaryEntry { lemma: "jestvovati", addition: "", transitive: true, imperfective: true },
    ],
    "jezditi" => &[
        VerbDictionaryEntry { lemma: "jezditi", addition: "", transitive: true, imperfective: true },
    ],
    "ježiti" => &[
        VerbDictionaryEntry { lemma: "ježiti", addition: "", transitive: true, imperfective: true },
    ],
    "jęti" => &[
        VerbDictionaryEntry { lemma: "jęti", addition: "(jme)", transitive: true, imperfective: true },
    ],
    "jęčati" => &[
        VerbDictionaryEntry { lemma: "jęčati", addition: "(jęči)", transitive: true, imperfective: true },
    ],
    "kaditi" => &[
        VerbDictionaryEntry { lemma: "kaditi", addition: "", transitive: true, imperfective: true },
    ],
    "kakati" => &[
        VerbDictionaryEntry { lemma: "kakati", addition: "", transitive: true, imperfective: true },
    ],
    "kaliti" => &[
        VerbDictionaryEntry { lemma: "kaliti", addition: "", transitive: true, imperfective: true },
    ],
    "kalkulovati" => &[
        VerbDictionaryEntry { lemma: "kalkulovati", addition: "", transitive: true, imperfective: true },
    ],
    "kamenovati" => &[
        VerbDictionaryEntry { lemma: "kamenovati", addition: "", transitive: true, imperfective: true },
    ],
    "kameněti" => &[
        VerbDictionaryEntry { lemma: "kameněti", addition: "", transitive: true, imperfective: true },
    ],
    "kanalizovati" => &[
        VerbDictionaryEntry { lemma: "kanalizovati", addition: "", transitive: true, imperfective: true },
    ],
    "kandidovati" => &[
        VerbDictionaryEntry { lemma: "kandidovati", addition: "", transitive: true, imperfective: true },
    ],
    "kanonizovati" => &[
        VerbDictionaryEntry { lemma: "kanonizovati", addition: "", transitive: true, imperfective: true },
    ],
    "kapati" => &[
        VerbDictionaryEntry { lemma: "kapati", addition: "", transitive: true, imperfective: true },
    ],
    "kapitulovati" => &[
        VerbDictionaryEntry { lemma: "kapitulovati", addition: "", transitive: true, imperfective: true },
    ],
    "kapnųti" => &[
        VerbDictionaryEntry { lemma: "kapnųti", addition: "", transitive: true, imperfective: false },
    ],
    "kaprizovati" => &[
        VerbDictionaryEntry { lemma: "kaprizovati", addition: "", transitive: true, imperfective: true },
    ],
    "karati" => &[
        VerbDictionaryEntry { lemma: "karati", addition: "", transitive: true, imperfective: true },
    ],
    "kastrovati" => &[
        VerbDictionaryEntry { lemma: "kastrovati", addition: "", transitive: true, imperfective: true },
    ],
    "kategorizovati" => &[
        VerbDictionaryEntry { lemma: "kategorizovati", addition: "", transitive: true, imperfective: true },
    ],
    "kazati" => &[
        VerbDictionaryEntry { lemma: "kazati", addition: "(kaže)", transitive: true, imperfective: true },
    ],
    "kaziti" => &[
        VerbDictionaryEntry { lemma: "kaziti", addition: "", transitive: true, imperfective: true },
    ],
    "kazniti" => &[
        VerbDictionaryEntry { lemma: "kazniti", addition: "", transitive: true, imperfective: true },
    ],
    "kašljati" => &[
        VerbDictionaryEntry { lemma: "kašljati", addition: "", transitive: true, imperfective: true },
    ],
    "klasifikovati" => &[
        VerbDictionaryEntry { lemma: "klasifikovati", addition: "", transitive: true, imperfective: true },
    ],
    "klasti" => &[
        VerbDictionaryEntry { lemma: "klasti", addition: "(klade)", transitive: true, imperfective: true },
    ],
    "klejiti" => &[
        VerbDictionaryEntry { lemma: "klejiti", addition: "", transitive: true, imperfective: true },
    ],
    "klevetati" => &[
        VerbDictionaryEntry { lemma: "klevetati", addition: "", transitive: true, imperfective: true },
    ],
    "klicati" => &[
        VerbDictionaryEntry { lemma: "klicati", addition: "(kliče)", transitive: true, imperfective: true },
    ],
    "klikati" => &[
        VerbDictionaryEntry { lemma: "klikati", addition: "", transitive: true, imperfective: true },
    ],
    "kliknųti" => &[
        VerbDictionaryEntry { lemma: "kliknųti", addition: "", transitive: true, imperfective: false },
    ],
    "kljunųti" => &[
        VerbDictionaryEntry { lemma: "kljunųti", addition: "", transitive: true, imperfective: false },
    ],
    "kljusati" => &[
        VerbDictionaryEntry { lemma: "kljusati", addition: "", transitive: true, imperfective: true },
    ],
    "kljuvati" => &[
        VerbDictionaryEntry { lemma: "kljuvati", addition: "", transitive: true, imperfective: true },
    ],
    "klokotati" => &[
        VerbDictionaryEntry { lemma: "klokotati", addition: "", transitive: true, imperfective: true },
    ],
    "kloniti" => &[
        VerbDictionaryEntry { lemma: "kloniti", addition: "", transitive: true, imperfective: true },
    ],
    "klåti" => &[
        VerbDictionaryEntry { lemma: "klåti", addition: "(kolje)", transitive: true, imperfective: true },
    ],
    "klåtiti" => &[
        VerbDictionaryEntry { lemma: "klåtiti", addition: "", transitive: true, imperfective: true },
    ],
    "klęknųti" => &[
        VerbDictionaryEntry { lemma: "klęknųti", addition: "", transitive: true, imperfective: false },
    ],
    "klęti" => &[
        VerbDictionaryEntry { lemma: "klęti", addition: "(klne)", transitive: true, imperfective: true },
    ],
    "klęčati" => &[
        VerbDictionaryEntry { lemma: "klęčati", addition: "(klęče)", transitive: true, imperfective: true },
    ],
    "klěskati" => &[
        VerbDictionaryEntry { lemma: "klěskati", addition: "", transitive: true, imperfective: true },
    ],
    "kodifikovati" => &[
        VerbDictionaryEntry { lemma: "kodifikovati", addition: "", transitive: true, imperfective: true },
    ],
    "kodovati" => &[
        VerbDictionaryEntry { lemma: "kodovati", addition: "", transitive: true, imperfective: true },
    ],
    "koketovati" => &[
        VerbDictionaryEntry { lemma: "koketovati", addition: "", transitive: true, imperfective: true },
    ],
    "kokodakati" => &[
        VerbDictionaryEntry { lemma: "kokodakati", addition: "", transitive: true, imperfective: true },
    ],
    "kolonizovati" => &[
        VerbDictionaryEntry { lemma: "kolonizovati", addition: "", transitive: true, imperfective: true },
    ],
    "kolorovati" => &[
        VerbDictionaryEntry { lemma: "kolorovati", addition: "", transitive: true, imperfective: true },
    ],
    "kolovati" => &[
        VerbDictionaryEntry { lemma: "kolovati", addition: "", transitive: true, imperfective: true },
    ],
    "kolędovati" => &[
        VerbDictionaryEntry { lemma: "kolędovati", addition: "", transitive: true, imperfective: true },
    ],
    "kolěbati" => &[
        VerbDictionaryEntry { lemma: "kolěbati", addition: "", transitive: true, imperfective: true },
    ],
    "kombinovati" => &[
        VerbDictionaryEntry { lemma: "kombinovati", addition: "", transitive: true, imperfective: true },
    ],
    "komentovati" => &[
        VerbDictionaryEntry { lemma: "komentovati", addition: "", transitive: true, imperfective: true },
    ],
    "kompensovati" => &[
        VerbDictionaryEntry { lemma: "kompensovati", addition: "", transitive: true, imperfective: true },
    ],
    "kompjuterizovati" => &[
        VerbDictionaryEntry { lemma: "kompjuterizovati", addition: "", transitive: true, imperfective: true },
    ],
    "komplikovati" => &[
        VerbDictionaryEntry { lemma: "komplikovati", addition: "", transitive: true, imperfective: true },
    ],
    "komponovati" => &[
        VerbDictionaryEntry { lemma: "komponovati", addition: "", transitive: true, imperfective: true },
    ],
    "komunikovati" => &[
        VerbDictionaryEntry { lemma: "komunikovati", addition: "", transitive: true, imperfective: true },
    ],
    "koncentrovati" => &[
        VerbDictionaryEntry { lemma: "koncentrovati", addition: "", transitive: true, imperfective: true },
    ],
    "konfiskovati" => &[
        VerbDictionaryEntry { lemma: "konfiskovati", addition: "", transitive: true, imperfective: true },
    ],
    "konfliktovati" => &[
        VerbDictionaryEntry { lemma: "konfliktovati", addition: "", transitive: true, imperfective: true },
    ],
    "konkurovati" => &[
        VerbDictionaryEntry { lemma: "konkurovati", addition: "", transitive: true, imperfective: true },
    ],
    "konservovati" => &[
        VerbDictionaryEntry { lemma: "konservovati", addition: "", transitive: true, imperfective: true },
    ],
    "konstruovati" => &[
        VerbDictionaryEntry { lemma: "konstruovati", addition: "", transitive: true, imperfective: true },
    ],
    "kontrabandovati" => &[
        VerbDictionaryEntry { lemma: "kontrabandovati", addition: "", transitive: true, imperfective: true },
    ],
    "kontrolovati" => &[
        VerbDictionaryEntry { lemma: "kontrolovati", addition: "", transitive: true, imperfective: true },
    ],
    "končati" => &[
        VerbDictionaryEntry { lemma: "končati", addition: "", transitive: true, imperfective: true },
    ],
    "kooperovati" => &[
        VerbDictionaryEntry { lemma: "kooperovati", addition: "", transitive: true, imperfective: true },
    ],
    "koordinovati" => &[
        VerbDictionaryEntry { lemma: "koordinovati", addition: "", transitive: true, imperfective: true },
    ],
    "kopati" => &[
        VerbDictionaryEntry { lemma: "kopati", addition: "", transitive: true, imperfective: true },
    ],
    "kopijovati" => &[
        VerbDictionaryEntry { lemma: "kopijovati", addition: "", transitive: true, imperfective: true },
    ],
    "kopirovati" => &[
        VerbDictionaryEntry { lemma: "kopirovati", addition: "", transitive: true, imperfective: true },
    ],
    "korelovati" => &[
        VerbDictionaryEntry { lemma: "korelovati", addition: "", transitive: true, imperfective: true },
    ],
    "korigovati" => &[
        VerbDictionaryEntry { lemma: "korigovati", addition: "", transitive: true, imperfective: true },
    ],
    "koristati" => &[
        VerbDictionaryEntry { lemma: "koristati", addition: "", transitive: true, imperfective: true },
    ],
    "koristiti" => &[
        VerbDictionaryEntry { lemma: "koristiti", addition: "", transitive: true, imperfective: true },
    ],
    "koriti" => &[
        VerbDictionaryEntry { lemma: "koriti", addition: "", transitive: true, imperfective: true },
    ],
    "koronovati" => &[
        VerbDictionaryEntry { lemma: "koronovati", addition: "", transitive: true, imperfective: true },
    ],
    "kositi" => &[
        VerbDictionaryEntry { lemma: "kositi", addition: "", transitive: true, imperfective: true },
    ],
    "kovati" => &[
        VerbDictionaryEntry { lemma: "kovati", addition: "", transitive: true, imperfective: true },
    ],
    "koštovati" => &[
        VerbDictionaryEntry { lemma: "koštovati", addition: "", transitive: true, imperfective: true },
    ],
    "krakati" => &[
        VerbDictionaryEntry { lemma: "krakati", addition: "", transitive: true, imperfective: true },
    ],
    "krasiti" => &[
        VerbDictionaryEntry { lemma: "krasiti", addition: "", transitive: true, imperfective: true },
    ],
    "krasti" => &[
        VerbDictionaryEntry { lemma: "krasti", addition: "(krade)", transitive: true, imperfective: true },
    ],
    "kresati" => &[
        VerbDictionaryEntry { lemma: "kresati", addition: "", transitive: true, imperfective: true },
    ],
    "krijumčariti" => &[
        VerbDictionaryEntry { lemma: "krijumčariti", addition: "", transitive: true, imperfective: true },
    ],
    "kriknųti" => &[
        VerbDictionaryEntry { lemma: "kriknųti", addition: "", transitive: true, imperfective: false },
    ],
    "kristalizovati" => &[
        VerbDictionaryEntry { lemma: "kristalizovati", addition: "", transitive: true, imperfective: true },
    ],
    "kritikovati" => &[
        VerbDictionaryEntry { lemma: "kritikovati", addition: "", transitive: true, imperfective: true },
    ],
    "krivditi" => &[
        VerbDictionaryEntry { lemma: "krivditi", addition: "", transitive: true, imperfective: true },
    ],
    "kriviti" => &[
        VerbDictionaryEntry { lemma: "kriviti", addition: "", transitive: true, imperfective: true },
    ],
    "kričati" => &[
        VerbDictionaryEntry { lemma: "kričati", addition: "(kriči)", transitive: true, imperfective: true },
    ],
    "krjakati" => &[
        VerbDictionaryEntry { lemma: "krjakati", addition: "", transitive: true, imperfective: true },
    ],
    "krmiti" => &[
        VerbDictionaryEntry { lemma: "krmiti", addition: "", transitive: true, imperfective: true },
    ],
    "krojiti" => &[
        VerbDictionaryEntry { lemma: "krojiti", addition: "", transitive: true, imperfective: true },
    ],
    "kropiti" => &[
        VerbDictionaryEntry { lemma: "kropiti", addition: "", transitive: true, imperfective: true },
    ],
    "krotiti" => &[
        VerbDictionaryEntry { lemma: "krotiti", addition: "", transitive: true, imperfective: true },
    ],
    "krvaviti" => &[
        VerbDictionaryEntry { lemma: "krvaviti", addition: "", transitive: true, imperfective: true },
    ],
    "kryti" => &[
        VerbDictionaryEntry { lemma: "kryti", addition: "", transitive: true, imperfective: true },
    ],
    "krčiti" => &[
        VerbDictionaryEntry { lemma: "krčiti", addition: "", transitive: true, imperfective: true },
    ],
    "krėstiti" => &[
        VerbDictionaryEntry { lemma: "krėstiti", addition: "", transitive: true, imperfective: true },
    ],
    "krěpiti" => &[
        VerbDictionaryEntry { lemma: "krěpiti", addition: "", transitive: true, imperfective: true },
    ],
    "krěpěti" => &[
        VerbDictionaryEntry { lemma: "krěpěti", addition: "", transitive: true, imperfective: true },
    ],
    "krųtiti" => &[
        VerbDictionaryEntry { lemma: "krųtiti", addition: "", transitive: true, imperfective: true },
    ],
    "krųžiti" => &[
        VerbDictionaryEntry { lemma: "krųžiti", addition: "", transitive: true, imperfective: true },
    ],
    "krȯšiti" => &[
        VerbDictionaryEntry { lemma: "krȯšiti", addition: "", transitive: true, imperfective: true },
    ],
    "kuhati" => &[
        VerbDictionaryEntry { lemma: "kuhati", addition: "", transitive: true, imperfective: true },
    ],
    "kukati" => &[
        VerbDictionaryEntry { lemma: "kukati", addition: "", transitive: true, imperfective: true },
    ],
    "kuljgati" => &[
        VerbDictionaryEntry { lemma: "kuljgati", addition: "", transitive: true, imperfective: true },
    ],
    "kulminovati" => &[
        VerbDictionaryEntry { lemma: "kulminovati", addition: "", transitive: true, imperfective: true },
    ],
    "kupiti" => &[
        VerbDictionaryEntry { lemma: "kupiti", addition: "", transitive: true, imperfective: false },
    ],
    "kupovati" => &[
        VerbDictionaryEntry { lemma: "kupovati", addition: "", transitive: true, imperfective: true },
    ],
    "kuriti" => &[
        VerbDictionaryEntry { lemma: "kuriti", addition: "", transitive: true, imperfective: true },
    ],
    "kustomizovati" => &[
        VerbDictionaryEntry { lemma: "kustomizovati", addition: "", transitive: true, imperfective: true },
    ],
    "kvakati" => &[
        VerbDictionaryEntry { lemma: "kvakati", addition: "", transitive: true, imperfective: true },
    ],
    "kvalifikovati" => &[
        VerbDictionaryEntry { lemma: "kvalifikovati", addition: "", transitive: true, imperfective: true },
    ],
    "kvasiti" => &[
        VerbDictionaryEntry { lemma: "kvasiti", addition: "", transitive: true, imperfective: true },
    ],
    "kvičati" => &[
        VerbDictionaryEntry { lemma: "kvičati", addition: "(kviče)", transitive: true, imperfective: true },
    ],
    "kvokati" => &[
        VerbDictionaryEntry { lemma: "kvokati", addition: "", transitive: true, imperfective: true },
    ],
    "kydati" => &[
        VerbDictionaryEntry { lemma: "kydati", addition: "", transitive: true, imperfective: true },
    ],
    "kydnųti" => &[
        VerbDictionaryEntry { lemma: "kydnųti", addition: "", transitive: true, imperfective: false },
    ],
    "kyhati" => &[
        VerbDictionaryEntry { lemma: "kyhati", addition: "", transitive: true, imperfective: true },
    ],
    "kyhnųti" => &[
        VerbDictionaryEntry { lemma: "kyhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "kymati" => &[
        VerbDictionaryEntry { lemma: "kymati", addition: "", transitive: true, imperfective: true },
    ],
    "kymnųti" => &[
        VerbDictionaryEntry { lemma: "kymnųti", addition: "", transitive: true, imperfective: false },
    ],
    "kypěti" => &[
        VerbDictionaryEntry { lemma: "kypěti", addition: "(kipi)", transitive: true, imperfective: true },
    ],
    "kysnųti" => &[
        VerbDictionaryEntry { lemma: "kysnųti", addition: "", transitive: true, imperfective: true },
    ],
    "kyvati" => &[
        VerbDictionaryEntry { lemma: "kyvati", addition: "", transitive: true, imperfective: true },
    ],
    "kyvnųti" => &[
        VerbDictionaryEntry { lemma: "kyvnųti", addition: "", transitive: true, imperfective: false },
    ],
    "kųpati" => &[
        VerbDictionaryEntry { lemma: "kųpati", addition: "", transitive: true, imperfective: true },
    ],
    "kųsati" => &[
        VerbDictionaryEntry { lemma: "kųsati", addition: "", transitive: true, imperfective: true },
    ],
    "kųsnųti" => &[
        VerbDictionaryEntry { lemma: "kųsnųti", addition: "", transitive: true, imperfective: false },
    ],
    "lajati" => &[
        VerbDictionaryEntry { lemma: "lajati", addition: "(laje)", transitive: true, imperfective: true },
    ],
    "lakovati" => &[
        VerbDictionaryEntry { lemma: "lakovati", addition: "", transitive: true, imperfective: true },
    ],
    "lamati" => &[
        VerbDictionaryEntry { lemma: "lamati", addition: "", transitive: true, imperfective: true },
    ],
    "lapati" => &[
        VerbDictionaryEntry { lemma: "lapati", addition: "", transitive: true, imperfective: true },
    ],
    "laskati" => &[
        VerbDictionaryEntry { lemma: "laskati", addition: "", transitive: true, imperfective: true },
    ],
    "latati" => &[
        VerbDictionaryEntry { lemma: "latati", addition: "", transitive: true, imperfective: true },
    ],
    "laziti" => &[
        VerbDictionaryEntry { lemma: "laziti", addition: "", transitive: true, imperfective: true },
    ],
    "leděniti" => &[
        VerbDictionaryEntry { lemma: "leděniti", addition: "", transitive: true, imperfective: true },
    ],
    "leděněti" => &[
        VerbDictionaryEntry { lemma: "leděněti", addition: "", transitive: true, imperfective: true },
    ],
    "legalizovati" => &[
        VerbDictionaryEntry { lemma: "legalizovati", addition: "", transitive: true, imperfective: true },
    ],
    "legti" => &[
        VerbDictionaryEntry { lemma: "legti", addition: "", transitive: true, imperfective: false },
    ],
    "lepetati" => &[
        VerbDictionaryEntry { lemma: "lepetati", addition: "", transitive: true, imperfective: true },
    ],
    "letěti" => &[
        VerbDictionaryEntry { lemma: "letěti", addition: "(leti)", transitive: true, imperfective: true },
    ],
    "ležati" => &[
        VerbDictionaryEntry { lemma: "ležati", addition: "(leži)", transitive: true, imperfective: true },
    ],
    "lgati" => &[
        VerbDictionaryEntry { lemma: "lgati", addition: "(lže)", transitive: true, imperfective: true },
    ],
    "liberalizovati" => &[
        VerbDictionaryEntry { lemma: "liberalizovati", addition: "", transitive: true, imperfective: true },
    ],
    "liceměriti" => &[
        VerbDictionaryEntry { lemma: "liceměriti", addition: "", transitive: true, imperfective: true },
    ],
    "likvidovati" => &[
        VerbDictionaryEntry { lemma: "likvidovati", addition: "", transitive: true, imperfective: true },
    ],
    "limitovati" => &[
        VerbDictionaryEntry { lemma: "limitovati", addition: "", transitive: true, imperfective: true },
    ],
    "linjati" => &[
        VerbDictionaryEntry { lemma: "linjati", addition: "", transitive: true, imperfective: true },
    ],
    "linčevati" => &[
        VerbDictionaryEntry { lemma: "linčevati", addition: "", transitive: true, imperfective: true },
    ],
    "liti" => &[
        VerbDictionaryEntry { lemma: "liti", addition: "(lije)", transitive: true, imperfective: true },
    ],
    "lizati" => &[
        VerbDictionaryEntry { lemma: "lizati", addition: "(liže)", transitive: true, imperfective: true },
    ],
    "liznųti" => &[
        VerbDictionaryEntry { lemma: "liznųti", addition: "", transitive: true, imperfective: false },
    ],
    "lišati" => &[
        VerbDictionaryEntry { lemma: "lišati", addition: "", transitive: true, imperfective: true },
    ],
    "lišiti" => &[
        VerbDictionaryEntry { lemma: "lišiti", addition: "", transitive: true, imperfective: false },
    ],
    "ljstiti" => &[
        VerbDictionaryEntry { lemma: "ljstiti", addition: "", transitive: true, imperfective: true },
    ],
    "ljubiti" => &[
        VerbDictionaryEntry { lemma: "ljubiti", addition: "", transitive: true, imperfective: true },
    ],
    "ljuljati" => &[
        VerbDictionaryEntry { lemma: "ljuljati", addition: "", transitive: true, imperfective: true },
    ],
    "lobovati" => &[
        VerbDictionaryEntry { lemma: "lobovati", addition: "", transitive: true, imperfective: true },
    ],
    "lojiti" => &[
        VerbDictionaryEntry { lemma: "lojiti", addition: "", transitive: true, imperfective: true },
    ],
    "lokati" => &[
        VerbDictionaryEntry { lemma: "lokati", addition: "", transitive: true, imperfective: true },
    ],
    "loknųti" => &[
        VerbDictionaryEntry { lemma: "loknųti", addition: "", transitive: true, imperfective: false },
    ],
    "lomiti" => &[
        VerbDictionaryEntry { lemma: "lomiti", addition: "", transitive: true, imperfective: true },
    ],
    "loskotati" => &[
        VerbDictionaryEntry { lemma: "loskotati", addition: "(loskoče)", transitive: true, imperfective: true },
    ],
    "loviti" => &[
        VerbDictionaryEntry { lemma: "loviti", addition: "", transitive: true, imperfective: true },
    ],
    "lupiti" => &[
        VerbDictionaryEntry { lemma: "lupiti", addition: "", transitive: true, imperfective: true },
    ],
    "luskati" => &[
        VerbDictionaryEntry { lemma: "luskati", addition: "", transitive: true, imperfective: true },
    ],
    "lučiti" => &[
        VerbDictionaryEntry { lemma: "lučiti", addition: "", transitive: true, imperfective: true },
    ],
    "luščiti" => &[
        VerbDictionaryEntry { lemma: "luščiti", addition: "", transitive: true, imperfective: true },
    ],
    "lysěti" => &[
        VerbDictionaryEntry { lemma: "lysěti", addition: "", transitive: true, imperfective: true },
    ],
    "lėskati" => &[
        VerbDictionaryEntry { lemma: "lėskati", addition: "", transitive: true, imperfective: true },
    ],
    "lėsknųti" => &[
        VerbDictionaryEntry { lemma: "lėsknųti", addition: "", transitive: true, imperfective: false },
    ],
    "lęgati" => &[
        VerbDictionaryEntry { lemma: "lęgati", addition: "", transitive: true, imperfective: true },
    ],
    "lęgti" => &[
        VerbDictionaryEntry { lemma: "lęgti", addition: "", transitive: true, imperfective: true },
    ],
    "lěpiti" => &[
        VerbDictionaryEntry { lemma: "lěpiti", addition: "", transitive: true, imperfective: true },
    ],
    "lětati" => &[
        VerbDictionaryEntry { lemma: "lětati", addition: "", transitive: true, imperfective: true },
    ],
    "lětovati" => &[
        VerbDictionaryEntry { lemma: "lětovati", addition: "", transitive: true, imperfective: true },
    ],
    "lězti" => &[
        VerbDictionaryEntry { lemma: "lězti", addition: "", transitive: true, imperfective: true },
    ],
    "lěčiti" => &[
        VerbDictionaryEntry { lemma: "lěčiti", addition: "", transitive: true, imperfective: true },
    ],
    "mahati" => &[
        VerbDictionaryEntry { lemma: "mahati", addition: "", transitive: true, imperfective: true },
    ],
    "mahnųti" => &[
        VerbDictionaryEntry { lemma: "mahnųti", addition: "", transitive: true, imperfective: false },
    ],
    "maljevati" => &[
        VerbDictionaryEntry { lemma: "maljevati", addition: "", transitive: true, imperfective: true },
    ],
    "maltretovati" => &[
        VerbDictionaryEntry { lemma: "maltretovati", addition: "", transitive: true, imperfective: true },
    ],
    "malěti" => &[
        VerbDictionaryEntry { lemma: "malěti", addition: "", transitive: true, imperfective: true },
    ],
    "mamiti" => &[
        VerbDictionaryEntry { lemma: "mamiti", addition: "", transitive: true, imperfective: true },
    ],
    "manevrovati" => &[
        VerbDictionaryEntry { lemma: "manevrovati", addition: "", transitive: true, imperfective: true },
    ],
    "manipulovati" => &[
        VerbDictionaryEntry { lemma: "manipulovati", addition: "", transitive: true, imperfective: true },
    ],
    "marginalizovati" => &[
        VerbDictionaryEntry { lemma: "marginalizovati", addition: "", transitive: true, imperfective: true },
    ],
    "marinovati" => &[
        VerbDictionaryEntry { lemma: "marinovati", addition: "", transitive: true, imperfective: true },
    ],
    "marširovati" => &[
        VerbDictionaryEntry { lemma: "marširovati", addition: "", transitive: true, imperfective: true },
    ],
    "masakrovati" => &[
        VerbDictionaryEntry { lemma: "masakrovati", addition: "", transitive: true, imperfective: true },
    ],
    "masirovati" => &[
        VerbDictionaryEntry { lemma: "masirovati", addition: "", transitive: true, imperfective: true },
    ],
    "maskovati" => &[
        VerbDictionaryEntry { lemma: "maskovati", addition: "", transitive: true, imperfective: true },
    ],
    "mastiti" => &[
        VerbDictionaryEntry { lemma: "mastiti", addition: "", transitive: true, imperfective: true },
    ],
    "masturbovati" => &[
        VerbDictionaryEntry { lemma: "masturbovati", addition: "", transitive: true, imperfective: true },
    ],
    "mazati" => &[
        VerbDictionaryEntry { lemma: "mazati", addition: "(maže)", transitive: true, imperfective: true },
    ],
    "meblovati" => &[
        VerbDictionaryEntry { lemma: "meblovati", addition: "", transitive: true, imperfective: true },
    ],
    "meditovati" => &[
        VerbDictionaryEntry { lemma: "meditovati", addition: "", transitive: true, imperfective: true },
    ],
    "mekati" => &[
        VerbDictionaryEntry { lemma: "mekati", addition: "", transitive: true, imperfective: true },
    ],
    "mesti" => &[
        VerbDictionaryEntry { lemma: "mesti", addition: "(mete)", transitive: true, imperfective: true },
    ],
    "metati" => &[
        VerbDictionaryEntry { lemma: "metati", addition: "", transitive: true, imperfective: true },
    ],
    "metnųti" => &[
        VerbDictionaryEntry { lemma: "metnųti", addition: "", transitive: true, imperfective: false },
    ],
    "mglati" => &[
        VerbDictionaryEntry { lemma: "mglati", addition: "", transitive: true, imperfective: true },
    ],
    "migati" => &[
        VerbDictionaryEntry { lemma: "migati", addition: "", transitive: true, imperfective: true },
    ],
    "mignųti" => &[
        VerbDictionaryEntry { lemma: "mignųti", addition: "", transitive: true, imperfective: false },
    ],
    "migrovati" => &[
        VerbDictionaryEntry { lemma: "migrovati", addition: "", transitive: true, imperfective: true },
    ],
    "milovati" => &[
        VerbDictionaryEntry { lemma: "milovati", addition: "", transitive: true, imperfective: true },
    ],
    "minimalizovati" => &[
        VerbDictionaryEntry { lemma: "minimalizovati", addition: "", transitive: true, imperfective: true },
    ],
    "minovati" => &[
        VerbDictionaryEntry { lemma: "minovati", addition: "", transitive: true, imperfective: true },
    ],
    "minųti" => &[
        VerbDictionaryEntry { lemma: "minųti", addition: "", transitive: true, imperfective: false },
    ],
    "miriti" => &[
        VerbDictionaryEntry { lemma: "miriti", addition: "", transitive: true, imperfective: true },
    ],
    "mjaukati" => &[
        VerbDictionaryEntry { lemma: "mjaukati", addition: "", transitive: true, imperfective: true },
    ],
    "mjauknųti" => &[
        VerbDictionaryEntry { lemma: "mjauknųti", addition: "", transitive: true, imperfective: false },
    ],
    "mljaskati" => &[
        VerbDictionaryEntry { lemma: "mljaskati", addition: "", transitive: true, imperfective: true },
    ],
    "mlåděti" => &[
        VerbDictionaryEntry { lemma: "mlåděti", addition: "", transitive: true, imperfective: true },
    ],
    "mlåtiti" => &[
        VerbDictionaryEntry { lemma: "mlåtiti", addition: "", transitive: true, imperfective: true },
    ],
    "mlěti" => &[
        VerbDictionaryEntry { lemma: "mlěti", addition: "(melje)", transitive: true, imperfective: true },
    ],
    "množiti" => &[
        VerbDictionaryEntry { lemma: "množiti", addition: "", transitive: true, imperfective: true },
    ],
    "mněti" => &[
        VerbDictionaryEntry { lemma: "mněti", addition: "(mni)", transitive: true, imperfective: true },
    ],
    "mněvati" => &[
        VerbDictionaryEntry { lemma: "mněvati", addition: "", transitive: true, imperfective: true },
    ],
    "mobilizovati" => &[
        VerbDictionaryEntry { lemma: "mobilizovati", addition: "", transitive: true, imperfective: true },
    ],
    "modelovati" => &[
        VerbDictionaryEntry { lemma: "modelovati", addition: "", transitive: true, imperfective: true },
    ],
    "modernizovati" => &[
        VerbDictionaryEntry { lemma: "modernizovati", addition: "", transitive: true, imperfective: true },
    ],
    "moderovati" => &[
        VerbDictionaryEntry { lemma: "moderovati", addition: "", transitive: true, imperfective: true },
    ],
    "modifikovati" => &[
        VerbDictionaryEntry { lemma: "modifikovati", addition: "", transitive: true, imperfective: true },
    ],
    "modriti" => &[
        VerbDictionaryEntry { lemma: "modriti", addition: "", transitive: true, imperfective: true },
    ],
    "modrěti" => &[
        VerbDictionaryEntry { lemma: "modrěti", addition: "", transitive: true, imperfective: true },
    ],
    "mogti" => &[
        VerbDictionaryEntry { lemma: "mogti", addition: "", transitive: false, imperfective: true },
    ],
    "moknųti" => &[
        VerbDictionaryEntry { lemma: "moknųti", addition: "", transitive: true, imperfective: true },
    ],
    "mokriti" => &[
        VerbDictionaryEntry { lemma: "mokriti", addition: "", transitive: true, imperfective: true },
    ],
    "mokrěti" => &[
        VerbDictionaryEntry { lemma: "mokrěti", addition: "", transitive: true, imperfective: true },
    ],
    "molestovati" => &[
        VerbDictionaryEntry { lemma: "molestovati", addition: "", transitive: true, imperfective: true },
    ],
    "montovati" => &[
        VerbDictionaryEntry { lemma: "montovati", addition: "", transitive: true, imperfective: true },
    ],
    "moralizovati" => &[
        VerbDictionaryEntry { lemma: "moralizovati", addition: "", transitive: true, imperfective: true },
    ],
    "morati" => &[
        VerbDictionaryEntry { lemma: "morati", addition: "", transitive: false, imperfective: true },
    ],
    "moriti" => &[
        VerbDictionaryEntry { lemma: "moriti", addition: "", transitive: true, imperfective: true },
    ],
    "motati" => &[
        VerbDictionaryEntry { lemma: "motati", addition: "", transitive: true, imperfective: true },
    ],
    "motivovati" => &[
        VerbDictionaryEntry { lemma: "motivovati", addition: "", transitive: true, imperfective: true },
    ],
    "moćněti" => &[
        VerbDictionaryEntry { lemma: "moćněti", addition: "", transitive: true, imperfective: true },
    ],
    "močiti" => &[
        VerbDictionaryEntry { lemma: "močiti", addition: "", transitive: true, imperfective: true },
    ],
    "mrdati" => &[
        VerbDictionaryEntry { lemma: "mrdati", addition: "", transitive: true, imperfective: true },
    ],
    "mrdnųti" => &[
        VerbDictionaryEntry { lemma: "mrdnųti", addition: "", transitive: true, imperfective: false },
    ],
    "mrmjati" => &[
        VerbDictionaryEntry { lemma: "mrmjati", addition: "", transitive: true, imperfective: true },
    ],
    "mrznųti" => &[
        VerbDictionaryEntry { lemma: "mrznųti", addition: "", transitive: true, imperfective: true },
    ],
    "mråziti" => &[
        VerbDictionaryEntry { lemma: "mråziti", addition: "", transitive: true, imperfective: true },
    ],
    "mråzosušati" => &[
        VerbDictionaryEntry { lemma: "mråzosušati", addition: "", transitive: true, imperfective: true },
    ],
    "mråzosušiti" => &[
        VerbDictionaryEntry { lemma: "mråzosušiti", addition: "", transitive: true, imperfective: false },
    ],
    "mrčati" => &[
        VerbDictionaryEntry { lemma: "mrčati", addition: "", transitive: true, imperfective: true },
    ],
    "mrščiti" => &[
        VerbDictionaryEntry { lemma: "mrščiti", addition: "", transitive: true, imperfective: true },
    ],
    "mstiti" => &[
        VerbDictionaryEntry { lemma: "mstiti", addition: "", transitive: true, imperfective: true },
    ],
    "mukati" => &[
        VerbDictionaryEntry { lemma: "mukati", addition: "", transitive: true, imperfective: true },
    ],
    "musěti" => &[
        VerbDictionaryEntry { lemma: "musěti", addition: "(musi)", transitive: false, imperfective: true },
    ],
    "mučati" => &[
        VerbDictionaryEntry { lemma: "mučati", addition: "", transitive: true, imperfective: true },
    ],
    "myliti" => &[
        VerbDictionaryEntry { lemma: "myliti", addition: "", transitive: true, imperfective: true },
    ],
    "mysliti" => &[
        VerbDictionaryEntry { lemma: "mysliti", addition: "", transitive: true, imperfective: true },
    ],
    "myti" => &[
        VerbDictionaryEntry { lemma: "myti", addition: "", transitive: true, imperfective: true },
    ],
    "myškovati" => &[
        VerbDictionaryEntry { lemma: "myškovati", addition: "", transitive: true, imperfective: true },
    ],
    "mėdliti" => &[
        VerbDictionaryEntry { lemma: "mėdliti", addition: "", transitive: true, imperfective: true },
    ],
    "mėčtati" => &[
        VerbDictionaryEntry { lemma: "mėčtati", addition: "", transitive: true, imperfective: true },
    ],
    "mękčeti" => &[
        VerbDictionaryEntry { lemma: "mękčeti", addition: "", transitive: true, imperfective: true },
    ],
    "mękčiti" => &[
        VerbDictionaryEntry { lemma: "mękčiti", addition: "", transitive: true, imperfective: true },
    ],
    "męti" => &[
        VerbDictionaryEntry { lemma: "męti", addition: "(mne)", transitive: true, imperfective: true },
    ],
    "męčkati" => &[
        VerbDictionaryEntry { lemma: "męčkati", addition: "", transitive: true, imperfective: true },
    ],
    "měnjati" => &[
        VerbDictionaryEntry { lemma: "měnjati", addition: "", transitive: true, imperfective: true },
    ],
    "měriti" => &[
        VerbDictionaryEntry { lemma: "měriti", addition: "", transitive: true, imperfective: true },
    ],
    "měsiti" => &[
        VerbDictionaryEntry { lemma: "měsiti", addition: "", transitive: true, imperfective: true },
    ],
    "mětati" => &[
        VerbDictionaryEntry { lemma: "mětati", addition: "", transitive: true, imperfective: true },
    ],
    "měšati" => &[
        VerbDictionaryEntry { lemma: "měšati", addition: "", transitive: true, imperfective: true },
    ],
    "mŕknųti" => &[
        VerbDictionaryEntry { lemma: "mŕknųti", addition: "", transitive: true, imperfective: true },
    ],
    "mŕtvěti" => &[
        VerbDictionaryEntry { lemma: "mŕtvěti", addition: "", transitive: true, imperfective: true },
    ],
    "mųdrěti" => &[
        VerbDictionaryEntry { lemma: "mųdrěti", addition: "", transitive: true, imperfective: true },
    ],
    "mųtiti" => &[
        VerbDictionaryEntry { lemma: "mųtiti", addition: "", transitive: true, imperfective: true },
    ],
    "mųtněti" => &[
        VerbDictionaryEntry { lemma: "mųtněti", addition: "", transitive: true, imperfective: true },
    ],
    "mųčiti" => &[
        VerbDictionaryEntry { lemma: "mųčiti", addition: "", transitive: true, imperfective: true },
    ],
    "mȯlknųti" => &[
        VerbDictionaryEntry { lemma: "mȯlknųti", addition: "", transitive: true, imperfective: true },
    ],
    "mȯlviti" => &[
        VerbDictionaryEntry { lemma: "mȯlviti", addition: "", transitive: true, imperfective: true },
    ],
    "mȯlčati" => &[
        VerbDictionaryEntry { lemma: "mȯlčati", addition: "(mȯlči)", transitive: true, imperfective: true },
    ],
    "nabajati" => &[
        VerbDictionaryEntry { lemma: "nabajati", addition: "", transitive: true, imperfective: false },
    ],
    "nabirati" => &[
        VerbDictionaryEntry { lemma: "nabirati", addition: "", transitive: true, imperfective: true },
    ],
    "nabiti" => &[
        VerbDictionaryEntry { lemma: "nabiti", addition: "(nabije)", transitive: true, imperfective: false },
    ],
    "nabivati" => &[
        VerbDictionaryEntry { lemma: "nabivati", addition: "", transitive: true, imperfective: true },
    ],
    "nabrati" => &[
        VerbDictionaryEntry { lemma: "nabrati", addition: "(nabere)", transitive: true, imperfective: false },
    ],
    "nabreknųti" => &[
        VerbDictionaryEntry { lemma: "nabreknųti", addition: "", transitive: true, imperfective: false },
    ],
    "nabuhati" => &[
        VerbDictionaryEntry { lemma: "nabuhati", addition: "", transitive: true, imperfective: true },
    ],
    "nabuhnųti" => &[
        VerbDictionaryEntry { lemma: "nabuhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "nabyti" => &[
        VerbDictionaryEntry { lemma: "nabyti", addition: "(nabųde)", transitive: true, imperfective: false },
    ],
    "nabyvati" => &[
        VerbDictionaryEntry { lemma: "nabyvati", addition: "", transitive: true, imperfective: true },
    ],
    "nabzděti" => &[
        VerbDictionaryEntry { lemma: "nabzděti", addition: "", transitive: true, imperfective: false },
    ],
    "nacionalizovati" => &[
        VerbDictionaryEntry { lemma: "nacionalizovati", addition: "", transitive: true, imperfective: true },
    ],
    "nadati" => &[
        VerbDictionaryEntry { lemma: "nadati", addition: "(nada)", transitive: true, imperfective: false },
    ],
    "nadavati" => &[
        VerbDictionaryEntry { lemma: "nadavati", addition: "", transitive: true, imperfective: true },
    ],
    "nadględati" => &[
        VerbDictionaryEntry { lemma: "nadględati", addition: "", transitive: false, imperfective: true },
    ],
    "nadględěti" => &[
        VerbDictionaryEntry { lemma: "nadględěti", addition: "(nadględi)", transitive: false, imperfective: false },
    ],
    "nadigrati" => &[
        VerbDictionaryEntry { lemma: "nadigrati", addition: "", transitive: true, imperfective: false },
    ],
    "nadigryvati" => &[
        VerbDictionaryEntry { lemma: "nadigryvati", addition: "", transitive: true, imperfective: true },
    ],
    "nadužiti" => &[
        VerbDictionaryEntry { lemma: "nadužiti", addition: "", transitive: true, imperfective: false },
    ],
    "naduživati" => &[
        VerbDictionaryEntry { lemma: "naduživati", addition: "", transitive: true, imperfective: true },
    ],
    "nadyhati" => &[
        VerbDictionaryEntry { lemma: "nadyhati", addition: "", transitive: true, imperfective: true },
    ],
    "nadzirati" => &[
        VerbDictionaryEntry { lemma: "nadzirati", addition: "", transitive: true, imperfective: true },
    ],
    "nadělati" => &[
        VerbDictionaryEntry { lemma: "nadělati", addition: "", transitive: true, imperfective: false },
    ],
    "naděti" => &[
        VerbDictionaryEntry { lemma: "naděti", addition: "", transitive: true, imperfective: false },
    ],
    "naděvati" => &[
        VerbDictionaryEntry { lemma: "naděvati", addition: "", transitive: true, imperfective: true },
    ],
    "nadųti" => &[
        VerbDictionaryEntry { lemma: "nadųti", addition: "(nadme)", transitive: true, imperfective: false },
    ],
    "nadųvati" => &[
        VerbDictionaryEntry { lemma: "nadųvati", addition: "", transitive: true, imperfective: true },
    ],
    "nadȯhnųti" => &[
        VerbDictionaryEntry { lemma: "nadȯhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "naganjati" => &[
        VerbDictionaryEntry { lemma: "naganjati", addition: "", transitive: true, imperfective: true },
    ],
    "nagnati" => &[
        VerbDictionaryEntry { lemma: "nagnati", addition: "(nagone)", transitive: true, imperfective: false },
    ],
    "nagnojiti" => &[
        VerbDictionaryEntry { lemma: "nagnojiti", addition: "", transitive: true, imperfective: false },
    ],
    "nagnųti" => &[
        VerbDictionaryEntry { lemma: "nagnųti", addition: "", transitive: true, imperfective: false },
    ],
    "nagovoriti" => &[
        VerbDictionaryEntry { lemma: "nagovoriti", addition: "", transitive: true, imperfective: false },
    ],
    "nagrađati" => &[
        VerbDictionaryEntry { lemma: "nagrađati", addition: "", transitive: true, imperfective: true },
    ],
    "nagromaditi" => &[
        VerbDictionaryEntry { lemma: "nagromaditi", addition: "", transitive: true, imperfective: false },
    ],
    "nagromađati" => &[
        VerbDictionaryEntry { lemma: "nagromađati", addition: "", transitive: true, imperfective: true },
    ],
    "nagråditi" => &[
        VerbDictionaryEntry { lemma: "nagråditi", addition: "", transitive: true, imperfective: false },
    ],
    "nagrěti" => &[
        VerbDictionaryEntry { lemma: "nagrěti", addition: "(nagrěje)", transitive: true, imperfective: false },
    ],
    "nagrěvati" => &[
        VerbDictionaryEntry { lemma: "nagrěvati", addition: "", transitive: true, imperfective: true },
    ],
    "nagybati" => &[
        VerbDictionaryEntry { lemma: "nagybati", addition: "", transitive: true, imperfective: true },
    ],
    "nahmuriti" => &[
        VerbDictionaryEntry { lemma: "nahmuriti", addition: "", transitive: true, imperfective: false },
    ],
    "nahoditi" => &[
        VerbDictionaryEntry { lemma: "nahoditi", addition: "", transitive: true, imperfective: true },
    ],
    "nahvatati" => &[
        VerbDictionaryEntry { lemma: "nahvatati", addition: "", transitive: true, imperfective: false },
    ],
    "naigrati" => &[
        VerbDictionaryEntry { lemma: "naigrati", addition: "", transitive: true, imperfective: false },
    ],
    "naigryvati" => &[
        VerbDictionaryEntry { lemma: "naigryvati", addition: "", transitive: true, imperfective: true },
    ],
    "najdti" => &[
        VerbDictionaryEntry { lemma: "najdti", addition: "(najde; našėl)", transitive: true, imperfective: false },
    ],
    "naježiti" => &[
        VerbDictionaryEntry { lemma: "naježiti", addition: "", transitive: true, imperfective: false },
    ],
    "najmati" => &[
        VerbDictionaryEntry { lemma: "najmati", addition: "", transitive: true, imperfective: true },
    ],
    "najęti" => &[
        VerbDictionaryEntry { lemma: "najęti", addition: "(najme)", transitive: true, imperfective: false },
    ],
    "nakalati" => &[
        VerbDictionaryEntry { lemma: "nakalati", addition: "", transitive: true, imperfective: true },
    ],
    "nakapati" => &[
        VerbDictionaryEntry { lemma: "nakapati", addition: "", transitive: true, imperfective: false },
    ],
    "nakazati" => &[
        VerbDictionaryEntry { lemma: "nakazati", addition: "(nakaže)", transitive: true, imperfective: false },
    ],
    "nakazyvati" => &[
        VerbDictionaryEntry { lemma: "nakazyvati", addition: "", transitive: true, imperfective: true },
    ],
    "nakladati" => &[
        VerbDictionaryEntry { lemma: "nakladati", addition: "", transitive: true, imperfective: true },
    ],
    "naklanjati" => &[
        VerbDictionaryEntry { lemma: "naklanjati", addition: "", transitive: true, imperfective: true },
    ],
    "nakloniti" => &[
        VerbDictionaryEntry { lemma: "nakloniti", addition: "", transitive: true, imperfective: false },
    ],
    "nakopati" => &[
        VerbDictionaryEntry { lemma: "nakopati", addition: "", transitive: true, imperfective: false },
    ],
    "nakopyvati" => &[
        VerbDictionaryEntry { lemma: "nakopyvati", addition: "", transitive: true, imperfective: true },
    ],
    "nakrmiti" => &[
        VerbDictionaryEntry { lemma: "nakrmiti", addition: "", transitive: true, imperfective: false },
    ],
    "nakryti" => &[
        VerbDictionaryEntry { lemma: "nakryti", addition: "", transitive: true, imperfective: false },
    ],
    "nakryvati" => &[
        VerbDictionaryEntry { lemma: "nakryvati", addition: "", transitive: true, imperfective: true },
    ],
    "nakydati" => &[
        VerbDictionaryEntry { lemma: "nakydati", addition: "", transitive: true, imperfective: false },
    ],
    "nakydyvati" => &[
        VerbDictionaryEntry { lemma: "nakydyvati", addition: "", transitive: true, imperfective: true },
    ],
    "nalagati" => &[
        VerbDictionaryEntry { lemma: "nalagati", addition: "", transitive: true, imperfective: true },
    ],
    "nalegati" => &[
        VerbDictionaryEntry { lemma: "nalegati", addition: "", transitive: true, imperfective: true },
    ],
    "nalegti" => &[
        VerbDictionaryEntry { lemma: "nalegti", addition: "", transitive: true, imperfective: false },
    ],
    "naležati" => &[
        VerbDictionaryEntry { lemma: "naležati", addition: "(naleži)", transitive: true, imperfective: true },
    ],
    "naliti" => &[
        VerbDictionaryEntry { lemma: "naliti", addition: "(nalije)", transitive: true, imperfective: true },
    ],
    "nalivati" => &[
        VerbDictionaryEntry { lemma: "nalivati", addition: "", transitive: true, imperfective: false },
    ],
    "naložiti" => &[
        VerbDictionaryEntry { lemma: "naložiti", addition: "", transitive: true, imperfective: false },
    ],
    "namastiti" => &[
        VerbDictionaryEntry { lemma: "namastiti", addition: "", transitive: true, imperfective: false },
    ],
    "namazati" => &[
        VerbDictionaryEntry { lemma: "namazati", addition: "(namaže)", transitive: true, imperfective: false },
    ],
    "namazyvati" => &[
        VerbDictionaryEntry { lemma: "namazyvati", addition: "", transitive: true, imperfective: true },
    ],
    "namašćati" => &[
        VerbDictionaryEntry { lemma: "namašćati", addition: "", transitive: true, imperfective: true },
    ],
    "namoknųti" => &[
        VerbDictionaryEntry { lemma: "namoknųti", addition: "", transitive: true, imperfective: false },
    ],
    "namokriti" => &[
        VerbDictionaryEntry { lemma: "namokriti", addition: "", transitive: true, imperfective: false },
    ],
    "namontovati" => &[
        VerbDictionaryEntry { lemma: "namontovati", addition: "", transitive: true, imperfective: false },
    ],
    "namotati" => &[
        VerbDictionaryEntry { lemma: "namotati", addition: "", transitive: true, imperfective: false },
    ],
    "namotyvati" => &[
        VerbDictionaryEntry { lemma: "namotyvati", addition: "", transitive: true, imperfective: true },
    ],
    "namočiti" => &[
        VerbDictionaryEntry { lemma: "namočiti", addition: "", transitive: true, imperfective: false },
    ],
    "namyliti" => &[
        VerbDictionaryEntry { lemma: "namyliti", addition: "", transitive: true, imperfective: false },
    ],
    "namyljati" => &[
        VerbDictionaryEntry { lemma: "namyljati", addition: "", transitive: true, imperfective: true },
    ],
    "naměriti" => &[
        VerbDictionaryEntry { lemma: "naměriti", addition: "", transitive: true, imperfective: false },
    ],
    "naměrjati" => &[
        VerbDictionaryEntry { lemma: "naměrjati", addition: "", transitive: true, imperfective: true },
    ],
    "nanesti" => &[
        VerbDictionaryEntry { lemma: "nanesti", addition: "", transitive: true, imperfective: false },
    ],
    "nanizati" => &[
        VerbDictionaryEntry { lemma: "nanizati", addition: "(naniže)", transitive: true, imperfective: false },
    ],
    "nanositi" => &[
        VerbDictionaryEntry { lemma: "nanositi", addition: "", transitive: true, imperfective: true },
    ],
    "naostriti" => &[
        VerbDictionaryEntry { lemma: "naostriti", addition: "", transitive: true, imperfective: false },
    ],
    "napadati" => &[
        VerbDictionaryEntry { lemma: "napadati", addition: "", transitive: true, imperfective: true },
    ],
    "napajati" => &[
        VerbDictionaryEntry { lemma: "napajati", addition: "", transitive: true, imperfective: true },
    ],
    "naparfumovati" => &[
        VerbDictionaryEntry { lemma: "naparfumovati", addition: "", transitive: true, imperfective: false },
    ],
    "napasti" => &[
        VerbDictionaryEntry { lemma: "napasti", addition: "(napade)", transitive: true, imperfective: false },
        VerbDictionaryEntry { lemma: "napasti", addition: "(napase)", transitive: true, imperfective: false },
    ],
    "napečatati" => &[
        VerbDictionaryEntry { lemma: "napečatati", addition: "", transitive: true, imperfective: false },
    ],
    "naphati" => &[
        VerbDictionaryEntry { lemma: "naphati", addition: "", transitive: true, imperfective: false },
    ],
    "napihati" => &[
        VerbDictionaryEntry { lemma: "napihati", addition: "", transitive: true, imperfective: true },
    ],
    "napinati" => &[
        VerbDictionaryEntry { lemma: "napinati", addition: "", transitive: true, imperfective: true },
    ],
    "napirati" => &[
        VerbDictionaryEntry { lemma: "napirati", addition: "", transitive: true, imperfective: true },
    ],
    "napisati" => &[
        VerbDictionaryEntry { lemma: "napisati", addition: "(napiše)", transitive: true, imperfective: false },
    ],
    "napljuvati" => &[
        VerbDictionaryEntry { lemma: "napljuvati", addition: "", transitive: true, imperfective: false },
    ],
    "napojiti" => &[
        VerbDictionaryEntry { lemma: "napojiti", addition: "", transitive: true, imperfective: false },
    ],
    "napominati" => &[
        VerbDictionaryEntry { lemma: "napominati", addition: "", transitive: true, imperfective: true },
    ],
    "napomněti" => &[
        VerbDictionaryEntry { lemma: "napomněti", addition: "(napomni)", transitive: true, imperfective: false },
    ],
    "napraviti" => &[
        VerbDictionaryEntry { lemma: "napraviti", addition: "", transitive: true, imperfective: false },
    ],
    "napravjati" => &[
        VerbDictionaryEntry { lemma: "napravjati", addition: "", transitive: true, imperfective: true },
    ],
    "naprěti" => &[
        VerbDictionaryEntry { lemma: "naprěti", addition: "(napre)", transitive: true, imperfective: false },
    ],
    "naprųžiti" => &[
        VerbDictionaryEntry { lemma: "naprųžiti", addition: "", transitive: true, imperfective: false },
    ],
    "napustiti" => &[
        VerbDictionaryEntry { lemma: "napustiti", addition: "", transitive: true, imperfective: false },
    ],
    "napušćati" => &[
        VerbDictionaryEntry { lemma: "napušćati", addition: "", transitive: true, imperfective: true },
    ],
    "napęti" => &[
        VerbDictionaryEntry { lemma: "napęti", addition: "(napne)", transitive: true, imperfective: false },
    ],
    "napŕti" => &[
        VerbDictionaryEntry { lemma: "napŕti", addition: "(napre)", transitive: true, imperfective: false },
    ],
    "napȯlniti" => &[
        VerbDictionaryEntry { lemma: "napȯlniti", addition: "", transitive: true, imperfective: false },
    ],
    "napȯlnjati" => &[
        VerbDictionaryEntry { lemma: "napȯlnjati", addition: "", transitive: true, imperfective: true },
    ],
    "narastati" => &[
        VerbDictionaryEntry { lemma: "narastati", addition: "", transitive: true, imperfective: true },
    ],
    "naroditi" => &[
        VerbDictionaryEntry { lemma: "naroditi", addition: "", transitive: true, imperfective: false },
    ],
    "narušati" => &[
        VerbDictionaryEntry { lemma: "narušati", addition: "", transitive: true, imperfective: true },
    ],
    "narušiti" => &[
        VerbDictionaryEntry { lemma: "narušiti", addition: "", transitive: true, imperfective: false },
    ],
    "narvati" => &[
        VerbDictionaryEntry { lemma: "narvati", addition: "(narve)", transitive: true, imperfective: false },
    ],
    "narysovati" => &[
        VerbDictionaryEntry { lemma: "narysovati", addition: "", transitive: true, imperfective: false },
    ],
    "naryvati" => &[
        VerbDictionaryEntry { lemma: "naryvati", addition: "", transitive: true, imperfective: true },
    ],
    "naråsti" => &[
        VerbDictionaryEntry { lemma: "naråsti", addition: "(naråste)", transitive: true, imperfective: false },
    ],
    "narěkati" => &[
        VerbDictionaryEntry { lemma: "narěkati", addition: "", transitive: true, imperfective: true },
    ],
    "narěkti" => &[
        VerbDictionaryEntry { lemma: "narěkti", addition: "", transitive: true, imperfective: false },
    ],
    "narězati" => &[
        VerbDictionaryEntry { lemma: "narězati", addition: "(narěže)", transitive: true, imperfective: true },
    ],
    "narųčati" => &[
        VerbDictionaryEntry { lemma: "narųčati", addition: "", transitive: true, imperfective: true },
    ],
    "narųčiti" => &[
        VerbDictionaryEntry { lemma: "narųčiti", addition: "", transitive: true, imperfective: false },
    ],
    "nasaditi" => &[
        VerbDictionaryEntry { lemma: "nasaditi", addition: "", transitive: true, imperfective: false },
    ],
    "nasađati" => &[
        VerbDictionaryEntry { lemma: "nasađati", addition: "", transitive: true, imperfective: true },
    ],
    "nascati" => &[
        VerbDictionaryEntry { lemma: "nascati", addition: "(nasci)", transitive: true, imperfective: false },
    ],
    "naseliti" => &[
        VerbDictionaryEntry { lemma: "naseliti", addition: "", transitive: true, imperfective: false },
    ],
    "naseljati" => &[
        VerbDictionaryEntry { lemma: "naseljati", addition: "", transitive: true, imperfective: true },
    ],
    "nasilovati" => &[
        VerbDictionaryEntry { lemma: "nasilovati", addition: "", transitive: true, imperfective: true },
    ],
    "naslađati" => &[
        VerbDictionaryEntry { lemma: "naslađati", addition: "", transitive: true, imperfective: true },
    ],
    "naslåditi" => &[
        VerbDictionaryEntry { lemma: "naslåditi", addition: "", transitive: true, imperfective: false },
    ],
    "naslěditi" => &[
        VerbDictionaryEntry { lemma: "naslěditi", addition: "", transitive: true, imperfective: false },
    ],
    "naslědovati" => &[
        VerbDictionaryEntry { lemma: "naslědovati", addition: "", transitive: true, imperfective: true },
    ],
    "nasmoliti" => &[
        VerbDictionaryEntry { lemma: "nasmoliti", addition: "", transitive: true, imperfective: false },
    ],
    "nastati" => &[
        VerbDictionaryEntry { lemma: "nastati", addition: "(nastane)", transitive: true, imperfective: false },
    ],
    "nastavati" => &[
        VerbDictionaryEntry { lemma: "nastavati", addition: "", transitive: true, imperfective: true },
    ],
    "nastaviti" => &[
        VerbDictionaryEntry { lemma: "nastaviti", addition: "", transitive: true, imperfective: false },
    ],
    "nastavjati" => &[
        VerbDictionaryEntry { lemma: "nastavjati", addition: "", transitive: true, imperfective: true },
    ],
    "nastignųti" => &[
        VerbDictionaryEntry { lemma: "nastignųti", addition: "", transitive: true, imperfective: false },
    ],
    "nastojati" => &[
        VerbDictionaryEntry { lemma: "nastojati", addition: "(nastoji)", transitive: true, imperfective: false },
    ],
    "nastojivati" => &[
        VerbDictionaryEntry { lemma: "nastojivati", addition: "", transitive: true, imperfective: true },
    ],
    "nastrajati" => &[
        VerbDictionaryEntry { lemma: "nastrajati", addition: "", transitive: true, imperfective: true },
    ],
    "nastrašiti" => &[
        VerbDictionaryEntry { lemma: "nastrašiti", addition: "", transitive: true, imperfective: false },
    ],
    "nastrojiti" => &[
        VerbDictionaryEntry { lemma: "nastrojiti", addition: "", transitive: true, imperfective: false },
    ],
    "nastųpati" => &[
        VerbDictionaryEntry { lemma: "nastųpati", addition: "", transitive: true, imperfective: true },
    ],
    "nastųpiti" => &[
        VerbDictionaryEntry { lemma: "nastųpiti", addition: "", transitive: true, imperfective: false },
    ],
    "nasypati" => &[
        VerbDictionaryEntry { lemma: "nasypati", addition: "", transitive: true, imperfective: false },
    ],
    "nasytiti" => &[
        VerbDictionaryEntry { lemma: "nasytiti", addition: "", transitive: true, imperfective: false },
    ],
    "nasyćati" => &[
        VerbDictionaryEntry { lemma: "nasyćati", addition: "", transitive: true, imperfective: false },
    ],
    "nasěkati" => &[
        VerbDictionaryEntry { lemma: "nasěkati", addition: "", transitive: true, imperfective: true },
    ],
    "nasěkti" => &[
        VerbDictionaryEntry { lemma: "nasěkti", addition: "", transitive: true, imperfective: false },
    ],
    "natipkati" => &[
        VerbDictionaryEntry { lemma: "natipkati", addition: "", transitive: true, imperfective: false },
    ],
    "natirati" => &[
        VerbDictionaryEntry { lemma: "natirati", addition: "", transitive: true, imperfective: true },
    ],
    "natiskati" => &[
        VerbDictionaryEntry { lemma: "natiskati", addition: "", transitive: true, imperfective: true },
    ],
    "natisknųti" => &[
        VerbDictionaryEntry { lemma: "natisknųti", addition: "", transitive: true, imperfective: false },
    ],
    "natočiti" => &[
        VerbDictionaryEntry { lemma: "natočiti", addition: "", transitive: true, imperfective: false },
    ],
    "natrěti" => &[
        VerbDictionaryEntry { lemma: "natrěti", addition: "(natre)", transitive: true, imperfective: false },
    ],
    "natvoriti" => &[
        VerbDictionaryEntry { lemma: "natvoriti", addition: "", transitive: true, imperfective: false },
    ],
    "natęgati" => &[
        VerbDictionaryEntry { lemma: "natęgati", addition: "", transitive: true, imperfective: true },
    ],
    "natęgnųti" => &[
        VerbDictionaryEntry { lemma: "natęgnųti", addition: "", transitive: true, imperfective: true },
    ],
    "natŕti" => &[
        VerbDictionaryEntry { lemma: "natŕti", addition: "(natre)", transitive: true, imperfective: false },
    ],
    "naučiti" => &[
        VerbDictionaryEntry { lemma: "naučiti", addition: "", transitive: true, imperfective: false },
    ],
    "navesti" => &[
        VerbDictionaryEntry { lemma: "navesti", addition: "(navede)", transitive: true, imperfective: false },
    ],
    "naviti" => &[
        VerbDictionaryEntry { lemma: "naviti", addition: "(navije)", transitive: true, imperfective: false },
    ],
    "navivati" => &[
        VerbDictionaryEntry { lemma: "navivati", addition: "", transitive: true, imperfective: true },
    ],
    "navlåžiti" => &[
        VerbDictionaryEntry { lemma: "navlåžiti", addition: "", transitive: true, imperfective: false },
    ],
    "navlěkati" => &[
        VerbDictionaryEntry { lemma: "navlěkati", addition: "", transitive: true, imperfective: true },
    ],
    "navlěkti" => &[
        VerbDictionaryEntry { lemma: "navlěkti", addition: "", transitive: true, imperfective: false },
    ],
    "navoditi" => &[
        VerbDictionaryEntry { lemma: "navoditi", addition: "", transitive: true, imperfective: true },
    ],
    "navodniti" => &[
        VerbDictionaryEntry { lemma: "navodniti", addition: "", transitive: true, imperfective: false },
    ],
    "navodnjati" => &[
        VerbDictionaryEntry { lemma: "navodnjati", addition: "", transitive: true, imperfective: true },
    ],
    "navoščiti" => &[
        VerbDictionaryEntry { lemma: "navoščiti", addition: "", transitive: true, imperfective: false },
    ],
    "navråžiti" => &[
        VerbDictionaryEntry { lemma: "navråžiti", addition: "", transitive: true, imperfective: false },
    ],
    "navęzati" => &[
        VerbDictionaryEntry { lemma: "navęzati", addition: "(navęže)", transitive: true, imperfective: false },
    ],
    "navęzyvati" => &[
        VerbDictionaryEntry { lemma: "navęzyvati", addition: "", transitive: true, imperfective: true },
    ],
    "navěditi" => &[
        VerbDictionaryEntry { lemma: "navěditi", addition: "", transitive: true, imperfective: false },
    ],
    "navěsiti" => &[
        VerbDictionaryEntry { lemma: "navěsiti", addition: "", transitive: true, imperfective: false },
    ],
    "navěđati" => &[
        VerbDictionaryEntry { lemma: "navěđati", addition: "", transitive: true, imperfective: true },
    ],
    "navěšati" => &[
        VerbDictionaryEntry { lemma: "navěšati", addition: "", transitive: true, imperfective: true },
    ],
    "nazdravjati" => &[
        VerbDictionaryEntry { lemma: "nazdravjati", addition: "", transitive: true, imperfective: true },
    ],
    "nazdråviti" => &[
        VerbDictionaryEntry { lemma: "nazdråviti", addition: "", transitive: true, imperfective: false },
    ],
    "naznačati" => &[
        VerbDictionaryEntry { lemma: "naznačati", addition: "", transitive: true, imperfective: true },
    ],
    "naznačiti" => &[
        VerbDictionaryEntry { lemma: "naznačiti", addition: "", transitive: true, imperfective: false },
    ],
    "nazvati" => &[
        VerbDictionaryEntry { lemma: "nazvati", addition: "(nazȯve)", transitive: true, imperfective: false },
    ],
    "nazyvati" => &[
        VerbDictionaryEntry { lemma: "nazyvati", addition: "", transitive: true, imperfective: true },
    ],
    "načaditi" => &[
        VerbDictionaryEntry { lemma: "načaditi", addition: "", transitive: true, imperfective: false },
    ],
    "načinati" => &[
        VerbDictionaryEntry { lemma: "načinati", addition: "", transitive: true, imperfective: true },
    ],
    "načrpati" => &[
        VerbDictionaryEntry { lemma: "načrpati", addition: "", transitive: true, imperfective: false },
    ],
    "načrtati" => &[
        VerbDictionaryEntry { lemma: "načrtati", addition: "", transitive: true, imperfective: false },
    ],
    "načęti" => &[
        VerbDictionaryEntry { lemma: "načęti", addition: "(načne)", transitive: true, imperfective: false },
    ],
    "našiti" => &[
        VerbDictionaryEntry { lemma: "našiti", addition: "(našije)", transitive: true, imperfective: false },
    ],
    "našivati" => &[
        VerbDictionaryEntry { lemma: "našivati", addition: "", transitive: true, imperfective: true },
    ],
    "našėptati" => &[
        VerbDictionaryEntry { lemma: "našėptati", addition: "(našėpće)", transitive: true, imperfective: false },
    ],
    "nedoględnųti" => &[
        VerbDictionaryEntry { lemma: "nedoględnųti", addition: "", transitive: true, imperfective: false },
    ],
    "nedoråzuměti" => &[
        VerbDictionaryEntry { lemma: "nedoråzuměti", addition: "", transitive: true, imperfective: false },
    ],
    "nedoråzuměvati" => &[
        VerbDictionaryEntry { lemma: "nedoråzuměvati", addition: "", transitive: true, imperfective: true },
    ],
    "nedostavati" => &[
        VerbDictionaryEntry { lemma: "nedostavati", addition: "", transitive: true, imperfective: true },
    ],
    "nehati" => &[
        VerbDictionaryEntry { lemma: "nehati", addition: "", transitive: true, imperfective: false },
    ],
    "nenaviděti" => &[
        VerbDictionaryEntry { lemma: "nenaviděti", addition: "(nenavidi)", transitive: true, imperfective: true },
    ],
    "nepokojiti" => &[
        VerbDictionaryEntry { lemma: "nepokojiti", addition: "", transitive: true, imperfective: true },
    ],
    "nesti" => &[
        VerbDictionaryEntry { lemma: "nesti", addition: "", transitive: true, imperfective: true },
    ],
    "nevtralizovati" => &[
        VerbDictionaryEntry { lemma: "nevtralizovati", addition: "", transitive: true, imperfective: true },
    ],
    "nezadovaljati" => &[
        VerbDictionaryEntry { lemma: "nezadovaljati", addition: "", transitive: true, imperfective: true },
    ],
    "nezadovoliti" => &[
        VerbDictionaryEntry { lemma: "nezadovoliti", addition: "", transitive: true, imperfective: false },
    ],
    "nizati" => &[
        VerbDictionaryEntry { lemma: "nizati", addition: "(niže)", transitive: true, imperfective: true },
    ],
    "niščiti" => &[
        VerbDictionaryEntry { lemma: "niščiti", addition: "", transitive: true, imperfective: true },
    ],
    "njuhati" => &[
        VerbDictionaryEntry { lemma: "njuhati", addition: "", transitive: true, imperfective: true },
    ],
    "njuhnųti" => &[
        VerbDictionaryEntry { lemma: "njuhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "nominovati" => &[
        VerbDictionaryEntry { lemma: "nominovati", addition: "", transitive: true, imperfective: true },
    ],
    "nositi" => &[
        VerbDictionaryEntry { lemma: "nositi", addition: "", transitive: true, imperfective: true },
    ],
    "noćevati" => &[
        VerbDictionaryEntry { lemma: "noćevati", addition: "", transitive: true, imperfective: true },
    ],
    "nuditi" => &[
        VerbDictionaryEntry { lemma: "nuditi", addition: "", transitive: true, imperfective: true },
    ],
    "nyrjati" => &[
        VerbDictionaryEntry { lemma: "nyrjati", addition: "", transitive: true, imperfective: true },
    ],
    "nyrnųti" => &[
        VerbDictionaryEntry { lemma: "nyrnųti", addition: "", transitive: true, imperfective: false },
    ],
    "něměti" => &[
        VerbDictionaryEntry { lemma: "něměti", addition: "", transitive: true, imperfective: true },
    ],
    "obagriti" => &[
        VerbDictionaryEntry { lemma: "obagriti", addition: "", transitive: true, imperfective: false },
    ],
    "obdariti" => &[
        VerbDictionaryEntry { lemma: "obdariti", addition: "", transitive: true, imperfective: false },
    ],
    "obdarjati" => &[
        VerbDictionaryEntry { lemma: "obdarjati", addition: "", transitive: true, imperfective: true },
    ],
    "obdirati" => &[
        VerbDictionaryEntry { lemma: "obdirati", addition: "", transitive: true, imperfective: true },
    ],
    "obdivjati" => &[
        VerbDictionaryEntry { lemma: "obdivjati", addition: "", transitive: true, imperfective: true },
    ],
    "obdrěti" => &[
        VerbDictionaryEntry { lemma: "obdrěti", addition: "(obdre)", transitive: true, imperfective: false },
    ],
    "obdumati" => &[
        VerbDictionaryEntry { lemma: "obdumati", addition: "", transitive: true, imperfective: false },
    ],
    "obdŕti" => &[
        VerbDictionaryEntry { lemma: "obdŕti", addition: "(obdre)", transitive: true, imperfective: false },
    ],
    "obezglåviti" => &[
        VerbDictionaryEntry { lemma: "obezglåviti", addition: "", transitive: true, imperfective: false },
    ],
    "obezglåvjati" => &[
        VerbDictionaryEntry { lemma: "obezglåvjati", addition: "", transitive: true, imperfective: true },
    ],
    "obezhrabrjati" => &[
        VerbDictionaryEntry { lemma: "obezhrabrjati", addition: "", transitive: true, imperfective: true },
    ],
    "obezhråbriti" => &[
        VerbDictionaryEntry { lemma: "obezhråbriti", addition: "", transitive: true, imperfective: false },
    ],
    "obezpokojiti" => &[
        VerbDictionaryEntry { lemma: "obezpokojiti", addition: "", transitive: true, imperfective: false },
    ],
    "obezsiliti" => &[
        VerbDictionaryEntry { lemma: "obezsiliti", addition: "", transitive: true, imperfective: false },
    ],
    "obezsiljati" => &[
        VerbDictionaryEntry { lemma: "obezsiljati", addition: "", transitive: true, imperfective: true },
    ],
    "obezuměti" => &[
        VerbDictionaryEntry { lemma: "obezuměti", addition: "", transitive: true, imperfective: false },
    ],
    "obezuměvati" => &[
        VerbDictionaryEntry { lemma: "obezuměvati", addition: "", transitive: true, imperfective: true },
    ],
    "obezčėstiti" => &[
        VerbDictionaryEntry { lemma: "obezčėstiti", addition: "", transitive: true, imperfective: false },
    ],
    "obglodati" => &[
        VerbDictionaryEntry { lemma: "obglodati", addition: "", transitive: true, imperfective: false },
    ],
    "obględati" => &[
        VerbDictionaryEntry { lemma: "obględati", addition: "", transitive: true, imperfective: true },
    ],
    "obględěti" => &[
        VerbDictionaryEntry { lemma: "obględěti", addition: "(obględi)", transitive: true, imperfective: false },
    ],
    "obgovarjati" => &[
        VerbDictionaryEntry { lemma: "obgovarjati", addition: "", transitive: true, imperfective: true },
    ],
    "obgovoriti" => &[
        VerbDictionaryEntry { lemma: "obgovoriti", addition: "", transitive: true, imperfective: false },
    ],
    "obgryzati" => &[
        VerbDictionaryEntry { lemma: "obgryzati", addition: "", transitive: true, imperfective: true },
    ],
    "obgryzti" => &[
        VerbDictionaryEntry { lemma: "obgryzti", addition: "", transitive: true, imperfective: false },
    ],
    "obhoditi" => &[
        VerbDictionaryEntry { lemma: "obhoditi", addition: "", transitive: true, imperfective: true },
    ],
    "obhvatiti" => &[
        VerbDictionaryEntry { lemma: "obhvatiti", addition: "", transitive: true, imperfective: false },
    ],
    "obhvaćati" => &[
        VerbDictionaryEntry { lemma: "obhvaćati", addition: "", transitive: true, imperfective: true },
    ],
    "obiděti" => &[
        VerbDictionaryEntry { lemma: "obiděti", addition: "", transitive: true, imperfective: false },
    ],
    "obimati" => &[
        VerbDictionaryEntry { lemma: "obimati", addition: "", transitive: true, imperfective: true },
    ],
    "obiđati" => &[
        VerbDictionaryEntry { lemma: "obiđati", addition: "", transitive: true, imperfective: true },
    ],
    "objasniti" => &[
        VerbDictionaryEntry { lemma: "objasniti", addition: "", transitive: true, imperfective: false },
    ],
    "objasnjati" => &[
        VerbDictionaryEntry { lemma: "objasnjati", addition: "", transitive: true, imperfective: true },
    ],
    "objaviti" => &[
        VerbDictionaryEntry { lemma: "objaviti", addition: "", transitive: true, imperfective: false },
    ],
    "objavjati" => &[
        VerbDictionaryEntry { lemma: "objavjati", addition: "", transitive: true, imperfective: true },
    ],
    "objediniti" => &[
        VerbDictionaryEntry { lemma: "objediniti", addition: "", transitive: true, imperfective: false },
    ],
    "objedinjati" => &[
        VerbDictionaryEntry { lemma: "objedinjati", addition: "", transitive: true, imperfective: true },
    ],
    "objehati" => &[
        VerbDictionaryEntry { lemma: "objehati", addition: "(objede)", transitive: true, imperfective: false },
    ],
    "objezditi" => &[
        VerbDictionaryEntry { lemma: "objezditi", addition: "", transitive: true, imperfective: false },
    ],
    "obježđati" => &[
        VerbDictionaryEntry { lemma: "obježđati", addition: "", transitive: true, imperfective: true },
    ],
    "objęti" => &[
        VerbDictionaryEntry { lemma: "objęti", addition: "(obȯjme)", transitive: true, imperfective: false },
    ],
    "obkaljati" => &[
        VerbDictionaryEntry { lemma: "obkaljati", addition: "", transitive: true, imperfective: true },
    ],
    "obkoliti" => &[
        VerbDictionaryEntry { lemma: "obkoliti", addition: "", transitive: true, imperfective: false },
    ],
    "obkradati" => &[
        VerbDictionaryEntry { lemma: "obkradati", addition: "", transitive: true, imperfective: true },
    ],
    "obkrasti" => &[
        VerbDictionaryEntry { lemma: "obkrasti", addition: "(obkrade)", transitive: true, imperfective: false },
    ],
    "oblagati" => &[
        VerbDictionaryEntry { lemma: "oblagati", addition: "", transitive: true, imperfective: true },
    ],
    "oblajati" => &[
        VerbDictionaryEntry { lemma: "oblajati", addition: "", transitive: true, imperfective: false },
    ],
    "oblajivati" => &[
        VerbDictionaryEntry { lemma: "oblajivati", addition: "", transitive: true, imperfective: true },
    ],
    "oblamyvati" => &[
        VerbDictionaryEntry { lemma: "oblamyvati", addition: "", transitive: true, imperfective: true },
    ],
    "oblačati" => &[
        VerbDictionaryEntry { lemma: "oblačati", addition: "", transitive: true, imperfective: true },
    ],
    "obleděněti" => &[
        VerbDictionaryEntry { lemma: "obleděněti", addition: "", transitive: true, imperfective: false },
    ],
    "obletěti" => &[
        VerbDictionaryEntry { lemma: "obletěti", addition: "(obleti)", transitive: true, imperfective: false },
    ],
    "oblinjati" => &[
        VerbDictionaryEntry { lemma: "oblinjati", addition: "", transitive: true, imperfective: false },
    ],
    "obliti" => &[
        VerbDictionaryEntry { lemma: "obliti", addition: "(oblije)", transitive: true, imperfective: false },
    ],
    "oblivati" => &[
        VerbDictionaryEntry { lemma: "oblivati", addition: "", transitive: true, imperfective: true },
    ],
    "oblizati" => &[
        VerbDictionaryEntry { lemma: "oblizati", addition: "(obliže)", transitive: true, imperfective: false },
    ],
    "oblizyvati" => &[
        VerbDictionaryEntry { lemma: "oblizyvati", addition: "", transitive: true, imperfective: true },
    ],
    "oblomiti" => &[
        VerbDictionaryEntry { lemma: "oblomiti", addition: "", transitive: true, imperfective: false },
    ],
    "obložiti" => &[
        VerbDictionaryEntry { lemma: "obložiti", addition: "", transitive: true, imperfective: false },
    ],
    "oblupiti" => &[
        VerbDictionaryEntry { lemma: "oblupiti", addition: "", transitive: true, imperfective: false },
    ],
    "obluščiti" => &[
        VerbDictionaryEntry { lemma: "obluščiti", addition: "", transitive: true, imperfective: false },
    ],
    "oblysěti" => &[
        VerbDictionaryEntry { lemma: "oblysěti", addition: "", transitive: true, imperfective: false },
    ],
    "oblåčiti" => &[
        VerbDictionaryEntry { lemma: "oblåčiti", addition: "", transitive: true, imperfective: false },
    ],
    "oblėgčati" => &[
        VerbDictionaryEntry { lemma: "oblėgčati", addition: "", transitive: true, imperfective: true },
    ],
    "oblėgčiti" => &[
        VerbDictionaryEntry { lemma: "oblėgčiti", addition: "", transitive: true, imperfective: false },
    ],
    "oblěkati" => &[
        VerbDictionaryEntry { lemma: "oblěkati", addition: "", transitive: true, imperfective: true },
    ],
    "oblěkti" => &[
        VerbDictionaryEntry { lemma: "oblěkti", addition: "", transitive: true, imperfective: false },
    ],
    "oblětati" => &[
        VerbDictionaryEntry { lemma: "oblětati", addition: "", transitive: true, imperfective: true },
    ],
    "obmanyvati" => &[
        VerbDictionaryEntry { lemma: "obmanyvati", addition: "", transitive: true, imperfective: true },
    ],
    "obmanųti" => &[
        VerbDictionaryEntry { lemma: "obmanųti", addition: "", transitive: true, imperfective: false },
    ],
    "obmazati" => &[
        VerbDictionaryEntry { lemma: "obmazati", addition: "(obmaže)", transitive: true, imperfective: false },
    ],
    "obmazyvati" => &[
        VerbDictionaryEntry { lemma: "obmazyvati", addition: "", transitive: true, imperfective: true },
    ],
    "obmeđati" => &[
        VerbDictionaryEntry { lemma: "obmeđati", addition: "", transitive: true, imperfective: true },
    ],
    "obmeđiti" => &[
        VerbDictionaryEntry { lemma: "obmeđiti", addition: "", transitive: true, imperfective: false },
    ],
    "obmotati" => &[
        VerbDictionaryEntry { lemma: "obmotati", addition: "", transitive: true, imperfective: false },
    ],
    "obmotyvati" => &[
        VerbDictionaryEntry { lemma: "obmotyvati", addition: "", transitive: true, imperfective: true },
    ],
    "obmysliti" => &[
        VerbDictionaryEntry { lemma: "obmysliti", addition: "", transitive: true, imperfective: false },
    ],
    "obmysljati" => &[
        VerbDictionaryEntry { lemma: "obmysljati", addition: "", transitive: true, imperfective: true },
    ],
    "obměniti" => &[
        VerbDictionaryEntry { lemma: "obměniti", addition: "", transitive: true, imperfective: false },
    ],
    "obměnjati" => &[
        VerbDictionaryEntry { lemma: "obměnjati", addition: "", transitive: true, imperfective: true },
    ],
    "obnarodovati" => &[
        VerbDictionaryEntry { lemma: "obnarodovati", addition: "", transitive: true, imperfective: true },
    ],
    "obnavjati" => &[
        VerbDictionaryEntry { lemma: "obnavjati", addition: "", transitive: true, imperfective: true },
    ],
    "obnažati" => &[
        VerbDictionaryEntry { lemma: "obnažati", addition: "", transitive: true, imperfective: true },
    ],
    "obnažiti" => &[
        VerbDictionaryEntry { lemma: "obnažiti", addition: "", transitive: true, imperfective: false },
    ],
    "obnesti" => &[
        VerbDictionaryEntry { lemma: "obnesti", addition: "", transitive: true, imperfective: false },
    ],
    "obniziti" => &[
        VerbDictionaryEntry { lemma: "obniziti", addition: "", transitive: true, imperfective: false },
    ],
    "obnižati" => &[
        VerbDictionaryEntry { lemma: "obnižati", addition: "", transitive: true, imperfective: true },
    ],
    "obnjuhati" => &[
        VerbDictionaryEntry { lemma: "obnjuhati", addition: "", transitive: true, imperfective: false },
    ],
    "obnjuhyvati" => &[
        VerbDictionaryEntry { lemma: "obnjuhyvati", addition: "", transitive: true, imperfective: true },
    ],
    "obnositi" => &[
        VerbDictionaryEntry { lemma: "obnositi", addition: "", transitive: true, imperfective: true },
    ],
    "obnoviti" => &[
        VerbDictionaryEntry { lemma: "obnoviti", addition: "", transitive: true, imperfective: false },
    ],
    "obobćati" => &[
        VerbDictionaryEntry { lemma: "obobćati", addition: "", transitive: true, imperfective: true },
    ],
    "obobćiti" => &[
        VerbDictionaryEntry { lemma: "obobćiti", addition: "", transitive: true, imperfective: false },
    ],
    "obogatiti" => &[
        VerbDictionaryEntry { lemma: "obogatiti", addition: "", transitive: true, imperfective: false },
    ],
    "obogatěti" => &[
        VerbDictionaryEntry { lemma: "obogatěti", addition: "", transitive: true, imperfective: false },
    ],
    "obogaćati" => &[
        VerbDictionaryEntry { lemma: "obogaćati", addition: "", transitive: true, imperfective: true },
    ],
    "obožati" => &[
        VerbDictionaryEntry { lemma: "obožati", addition: "", transitive: true, imperfective: true },
    ],
    "obradovati" => &[
        VerbDictionaryEntry { lemma: "obradovati", addition: "", transitive: true, imperfective: false },
    ],
    "obranjati" => &[
        VerbDictionaryEntry { lemma: "obranjati", addition: "", transitive: true, imperfective: true },
    ],
    "obraćati" => &[
        VerbDictionaryEntry { lemma: "obraćati", addition: "", transitive: true, imperfective: true },
    ],
    "obriti" => &[
        VerbDictionaryEntry { lemma: "obriti", addition: "(obrije)", transitive: true, imperfective: false },
    ],
    "obrvati" => &[
        VerbDictionaryEntry { lemma: "obrvati", addition: "(obrve)", transitive: true, imperfective: false },
    ],
    "obrysovati" => &[
        VerbDictionaryEntry { lemma: "obrysovati", addition: "", transitive: true, imperfective: true },
    ],
    "obryvati" => &[
        VerbDictionaryEntry { lemma: "obryvati", addition: "", transitive: true, imperfective: true },
    ],
    "obryzgati" => &[
        VerbDictionaryEntry { lemma: "obryzgati", addition: "", transitive: true, imperfective: false },
    ],
    "obråbotati" => &[
        VerbDictionaryEntry { lemma: "obråbotati", addition: "", transitive: true, imperfective: false },
    ],
    "obråbotyvati" => &[
        VerbDictionaryEntry { lemma: "obråbotyvati", addition: "", transitive: true, imperfective: true },
    ],
    "obråniti" => &[
        VerbDictionaryEntry { lemma: "obråniti", addition: "", transitive: true, imperfective: false },
    ],
    "obråtiti" => &[
        VerbDictionaryEntry { lemma: "obråtiti", addition: "", transitive: true, imperfective: false },
    ],
    "obrěmeniti" => &[
        VerbDictionaryEntry { lemma: "obrěmeniti", addition: "", transitive: true, imperfective: false },
    ],
    "obrěmenjati" => &[
        VerbDictionaryEntry { lemma: "obrěmenjati", addition: "", transitive: true, imperfective: true },
    ],
    "obrězati" => &[
        VerbDictionaryEntry { lemma: "obrězati", addition: "(obrěže)", transitive: true, imperfective: false },
    ],
    "obsaditi" => &[
        VerbDictionaryEntry { lemma: "obsaditi", addition: "", transitive: true, imperfective: false },
    ],
    "obsađati" => &[
        VerbDictionaryEntry { lemma: "obsađati", addition: "", transitive: true, imperfective: true },
    ],
    "obscati" => &[
        VerbDictionaryEntry { lemma: "obscati", addition: "(obsci)", transitive: true, imperfective: false },
    ],
    "observovati" => &[
        VerbDictionaryEntry { lemma: "observovati", addition: "", transitive: true, imperfective: true },
    ],
    "obslugovati" => &[
        VerbDictionaryEntry { lemma: "obslugovati", addition: "", transitive: true, imperfective: true },
    ],
    "obslužiti" => &[
        VerbDictionaryEntry { lemma: "obslužiti", addition: "", transitive: true, imperfective: false },
    ],
    "obslědovati" => &[
        VerbDictionaryEntry { lemma: "obslědovati", addition: "", transitive: true, imperfective: true },
    ],
    "obsrati" => &[
        VerbDictionaryEntry { lemma: "obsrati", addition: "", transitive: true, imperfective: false },
    ],
    "obstajati" => &[
        VerbDictionaryEntry { lemma: "obstajati", addition: "", transitive: true, imperfective: true },
    ],
    "obstanavjati" => &[
        VerbDictionaryEntry { lemma: "obstanavjati", addition: "", transitive: true, imperfective: true },
    ],
    "obstųpati" => &[
        VerbDictionaryEntry { lemma: "obstųpati", addition: "", transitive: true, imperfective: true },
    ],
    "obstųpiti" => &[
        VerbDictionaryEntry { lemma: "obstųpiti", addition: "", transitive: true, imperfective: false },
    ],
    "obsvětliti" => &[
        VerbDictionaryEntry { lemma: "obsvětliti", addition: "", transitive: true, imperfective: false },
    ],
    "obsvětljati" => &[
        VerbDictionaryEntry { lemma: "obsvětljati", addition: "", transitive: true, imperfective: true },
    ],
    "obsyhati" => &[
        VerbDictionaryEntry { lemma: "obsyhati", addition: "", transitive: true, imperfective: true },
    ],
    "obsypati" => &[
        VerbDictionaryEntry { lemma: "obsypati", addition: "", transitive: true, imperfective: false },
    ],
    "obsypyvati" => &[
        VerbDictionaryEntry { lemma: "obsypyvati", addition: "", transitive: true, imperfective: true },
    ],
    "obsęgati" => &[
        VerbDictionaryEntry { lemma: "obsęgati", addition: "", transitive: true, imperfective: true },
    ],
    "obsęgnųti" => &[
        VerbDictionaryEntry { lemma: "obsęgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "obsějati" => &[
        VerbDictionaryEntry { lemma: "obsějati", addition: "(obsěje)", transitive: true, imperfective: false },
    ],
    "obsějivati" => &[
        VerbDictionaryEntry { lemma: "obsějivati", addition: "", transitive: true, imperfective: true },
    ],
    "obsěkati" => &[
        VerbDictionaryEntry { lemma: "obsěkati", addition: "", transitive: true, imperfective: true },
    ],
    "obsěkti" => &[
        VerbDictionaryEntry { lemma: "obsěkti", addition: "", transitive: true, imperfective: false },
    ],
    "obsȯhnųti" => &[
        VerbDictionaryEntry { lemma: "obsȯhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "obtestovati" => &[
        VerbDictionaryEntry { lemma: "obtestovati", addition: "", transitive: true, imperfective: false },
    ],
    "obtirati" => &[
        VerbDictionaryEntry { lemma: "obtirati", addition: "", transitive: true, imperfective: true },
    ],
    "obtrěti" => &[
        VerbDictionaryEntry { lemma: "obtrěti", addition: "(obtre)", transitive: true, imperfective: false },
    ],
    "obtęžati" => &[
        VerbDictionaryEntry { lemma: "obtęžati", addition: "", transitive: true, imperfective: true },
    ],
    "obtęžiti" => &[
        VerbDictionaryEntry { lemma: "obtęžiti", addition: "", transitive: true, imperfective: false },
    ],
    "obtŕti" => &[
        VerbDictionaryEntry { lemma: "obtŕti", addition: "(obtre)", transitive: true, imperfective: false },
    ],
    "obuzdati" => &[
        VerbDictionaryEntry { lemma: "obuzdati", addition: "", transitive: true, imperfective: false },
    ],
    "obuzdyvati" => &[
        VerbDictionaryEntry { lemma: "obuzdyvati", addition: "", transitive: true, imperfective: true },
    ],
    "obučati" => &[
        VerbDictionaryEntry { lemma: "obučati", addition: "", transitive: true, imperfective: true },
    ],
    "obučiti" => &[
        VerbDictionaryEntry { lemma: "obučiti", addition: "", transitive: true, imperfective: false },
    ],
    "obvadnjati" => &[
        VerbDictionaryEntry { lemma: "obvadnjati", addition: "", transitive: true, imperfective: true },
    ],
    "obvažati" => &[
        VerbDictionaryEntry { lemma: "obvažati", addition: "", transitive: true, imperfective: true },
    ],
    "obvažiti" => &[
        VerbDictionaryEntry { lemma: "obvažiti", addition: "", transitive: true, imperfective: false },
    ],
    "obvesti" => &[
        VerbDictionaryEntry { lemma: "obvesti", addition: "(obvede)", transitive: true, imperfective: false },
    ],
    "obviti" => &[
        VerbDictionaryEntry { lemma: "obviti", addition: "(obvije)", transitive: true, imperfective: false },
    ],
    "obvivati" => &[
        VerbDictionaryEntry { lemma: "obvivati", addition: "", transitive: true, imperfective: true },
    ],
    "obvoditi" => &[
        VerbDictionaryEntry { lemma: "obvoditi", addition: "", transitive: true, imperfective: true },
    ],
    "obvodniti" => &[
        VerbDictionaryEntry { lemma: "obvodniti", addition: "", transitive: true, imperfective: false },
    ],
    "obvęzati" => &[
        VerbDictionaryEntry { lemma: "obvęzati", addition: "(obvęže)", transitive: true, imperfective: false },
    ],
    "obvęzyvati" => &[
        VerbDictionaryEntry { lemma: "obvęzyvati", addition: "", transitive: true, imperfective: true },
    ],
    "obvěstiti" => &[
        VerbDictionaryEntry { lemma: "obvěstiti", addition: "", transitive: true, imperfective: false },
    ],
    "obvěšćati" => &[
        VerbDictionaryEntry { lemma: "obvěšćati", addition: "", transitive: true, imperfective: true },
    ],
    "obyvati" => &[
        VerbDictionaryEntry { lemma: "obyvati", addition: "", transitive: true, imperfective: true },
    ],
    "obćiti" => &[
        VerbDictionaryEntry { lemma: "obćiti", addition: "", transitive: true, imperfective: true },
    ],
    "občudovati" => &[
        VerbDictionaryEntry { lemma: "občudovati", addition: "", transitive: true, imperfective: true },
    ],
    "obědati" => &[
        VerbDictionaryEntry { lemma: "obědati", addition: "", transitive: true, imperfective: true },
    ],
    "oběgati" => &[
        VerbDictionaryEntry { lemma: "oběgati", addition: "", transitive: true, imperfective: false },
    ],
    "oběliti" => &[
        VerbDictionaryEntry { lemma: "oběliti", addition: "", transitive: true, imperfective: false },
    ],
    "obělěti" => &[
        VerbDictionaryEntry { lemma: "obělěti", addition: "", transitive: true, imperfective: false },
    ],
    "oběćati" => &[
        VerbDictionaryEntry { lemma: "oběćati", addition: "", transitive: true, imperfective: false },
    ],
    "oběćivati" => &[
        VerbDictionaryEntry { lemma: "oběćivati", addition: "", transitive: true, imperfective: true },
    ],
    "obŕnųti" => &[
        VerbDictionaryEntry { lemma: "obŕnųti", addition: "", transitive: true, imperfective: false },
    ],
    "obžegti" => &[
        VerbDictionaryEntry { lemma: "obžegti", addition: "(obžže)", transitive: true, imperfective: false },
    ],
    "obžigati" => &[
        VerbDictionaryEntry { lemma: "obžigati", addition: "", transitive: true, imperfective: true },
    ],
    "obȯjdti" => &[
        VerbDictionaryEntry { lemma: "obȯjdti", addition: "(obȯjde; obšėl)", transitive: true, imperfective: false },
    ],
    "ocěniti" => &[
        VerbDictionaryEntry { lemma: "ocěniti", addition: "", transitive: true, imperfective: false },
    ],
    "odbirati" => &[
        VerbDictionaryEntry { lemma: "odbirati", addition: "", transitive: true, imperfective: true },
    ],
    "odbiti" => &[
        VerbDictionaryEntry { lemma: "odbiti", addition: "(odbije)", transitive: true, imperfective: false },
    ],
    "odbivati" => &[
        VerbDictionaryEntry { lemma: "odbivati", addition: "", transitive: true, imperfective: true },
    ],
    "odbrati" => &[
        VerbDictionaryEntry { lemma: "odbrati", addition: "(odbere)", transitive: true, imperfective: false },
    ],
    "odběgati" => &[
        VerbDictionaryEntry { lemma: "odběgati", addition: "", transitive: true, imperfective: true },
    ],
    "odběgti" => &[
        VerbDictionaryEntry { lemma: "odběgti", addition: "(odběži)", transitive: true, imperfective: false },
    ],
    "oddaliti" => &[
        VerbDictionaryEntry { lemma: "oddaliti", addition: "", transitive: true, imperfective: false },
    ],
    "oddaljati" => &[
        VerbDictionaryEntry { lemma: "oddaljati", addition: "", transitive: true, imperfective: true },
    ],
    "oddati" => &[
        VerbDictionaryEntry { lemma: "oddati", addition: "(odda)", transitive: true, imperfective: false },
    ],
    "oddavati" => &[
        VerbDictionaryEntry { lemma: "oddavati", addition: "", transitive: true, imperfective: true },
    ],
    "oddirati" => &[
        VerbDictionaryEntry { lemma: "oddirati", addition: "", transitive: true, imperfective: true },
    ],
    "oddrěti" => &[
        VerbDictionaryEntry { lemma: "oddrěti", addition: "(oddre)", transitive: true, imperfective: false },
    ],
    "oddyhati" => &[
        VerbDictionaryEntry { lemma: "oddyhati", addition: "", transitive: true, imperfective: true },
    ],
    "odděliti" => &[
        VerbDictionaryEntry { lemma: "odděliti", addition: "", transitive: true, imperfective: false },
    ],
    "odděljati" => &[
        VerbDictionaryEntry { lemma: "odděljati", addition: "", transitive: true, imperfective: true },
    ],
    "oddŕti" => &[
        VerbDictionaryEntry { lemma: "oddŕti", addition: "(oddre)", transitive: true, imperfective: false },
    ],
    "oddȯhnųti" => &[
        VerbDictionaryEntry { lemma: "oddȯhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "odgadati" => &[
        VerbDictionaryEntry { lemma: "odgadati", addition: "", transitive: true, imperfective: false },
    ],
    "odgadyvati" => &[
        VerbDictionaryEntry { lemma: "odgadyvati", addition: "", transitive: true, imperfective: true },
    ],
    "odgnųti" => &[
        VerbDictionaryEntry { lemma: "odgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "odgovarjati" => &[
        VerbDictionaryEntry { lemma: "odgovarjati", addition: "", transitive: true, imperfective: true },
    ],
    "odgovoriti" => &[
        VerbDictionaryEntry { lemma: "odgovoriti", addition: "", transitive: true, imperfective: false },
    ],
    "odgrebati" => &[
        VerbDictionaryEntry { lemma: "odgrebati", addition: "", transitive: true, imperfective: true },
    ],
    "odgrebti" => &[
        VerbDictionaryEntry { lemma: "odgrebti", addition: "", transitive: true, imperfective: false },
    ],
    "odgryzati" => &[
        VerbDictionaryEntry { lemma: "odgryzati", addition: "", transitive: true, imperfective: true },
    ],
    "odgryzti" => &[
        VerbDictionaryEntry { lemma: "odgryzti", addition: "", transitive: true, imperfective: false },
    ],
    "odgybati" => &[
        VerbDictionaryEntry { lemma: "odgybati", addition: "", transitive: true, imperfective: true },
    ],
    "odhoditi" => &[
        VerbDictionaryEntry { lemma: "odhoditi", addition: "", transitive: true, imperfective: true },
    ],
    "odidti" => &[
        VerbDictionaryEntry { lemma: "odidti", addition: "(odide; odšėl)", transitive: true, imperfective: false },
    ],
    "odimati" => &[
        VerbDictionaryEntry { lemma: "odimati", addition: "", transitive: true, imperfective: true },
    ],
    "odjehati" => &[
        VerbDictionaryEntry { lemma: "odjehati", addition: "(odjede)", transitive: true, imperfective: false },
    ],
    "odježđati" => &[
        VerbDictionaryEntry { lemma: "odježđati", addition: "", transitive: true, imperfective: true },
    ],
    "odjęti" => &[
        VerbDictionaryEntry { lemma: "odjęti", addition: "(odȯjme)", transitive: true, imperfective: false },
    ],
    "odkazati" => &[
        VerbDictionaryEntry { lemma: "odkazati", addition: "(odkaže)", transitive: true, imperfective: false },
    ],
    "odkazyvati" => &[
        VerbDictionaryEntry { lemma: "odkazyvati", addition: "", transitive: true, imperfective: true },
    ],
    "odkladati" => &[
        VerbDictionaryEntry { lemma: "odkladati", addition: "", transitive: true, imperfective: true },
    ],
    "odklanjati" => &[
        VerbDictionaryEntry { lemma: "odklanjati", addition: "", transitive: true, imperfective: true },
    ],
    "odklejati" => &[
        VerbDictionaryEntry { lemma: "odklejati", addition: "", transitive: true, imperfective: true },
    ],
    "odklejiti" => &[
        VerbDictionaryEntry { lemma: "odklejiti", addition: "", transitive: true, imperfective: false },
    ],
    "odključati" => &[
        VerbDictionaryEntry { lemma: "odključati", addition: "", transitive: true, imperfective: true },
    ],
    "odključiti" => &[
        VerbDictionaryEntry { lemma: "odključiti", addition: "", transitive: true, imperfective: false },
    ],
    "odkloniti" => &[
        VerbDictionaryEntry { lemma: "odkloniti", addition: "", transitive: true, imperfective: false },
    ],
    "odkryti" => &[
        VerbDictionaryEntry { lemma: "odkryti", addition: "", transitive: true, imperfective: false },
    ],
    "odkryvati" => &[
        VerbDictionaryEntry { lemma: "odkryvati", addition: "", transitive: true, imperfective: true },
    ],
    "odkupiti" => &[
        VerbDictionaryEntry { lemma: "odkupiti", addition: "", transitive: true, imperfective: false },
    ],
    "odkydati" => &[
        VerbDictionaryEntry { lemma: "odkydati", addition: "", transitive: true, imperfective: true },
    ],
    "odkydnųti" => &[
        VerbDictionaryEntry { lemma: "odkydnųti", addition: "", transitive: true, imperfective: false },
    ],
    "odkųsiti" => &[
        VerbDictionaryEntry { lemma: "odkųsiti", addition: "", transitive: true, imperfective: false },
    ],
    "odkųšati" => &[
        VerbDictionaryEntry { lemma: "odkųšati", addition: "", transitive: true, imperfective: true },
    ],
    "odlamyvati" => &[
        VerbDictionaryEntry { lemma: "odlamyvati", addition: "", transitive: true, imperfective: true },
    ],
    "odletěti" => &[
        VerbDictionaryEntry { lemma: "odletěti", addition: "(odleti)", transitive: true, imperfective: false },
    ],
    "odliti" => &[
        VerbDictionaryEntry { lemma: "odliti", addition: "(odlije)", transitive: true, imperfective: false },
    ],
    "odlivati" => &[
        VerbDictionaryEntry { lemma: "odlivati", addition: "", transitive: true, imperfective: true },
    ],
    "odličati" => &[
        VerbDictionaryEntry { lemma: "odličati", addition: "", transitive: true, imperfective: true },
    ],
    "odličiti" => &[
        VerbDictionaryEntry { lemma: "odličiti", addition: "", transitive: true, imperfective: false },
    ],
    "odlomiti" => &[
        VerbDictionaryEntry { lemma: "odlomiti", addition: "", transitive: true, imperfective: false },
    ],
    "odložiti" => &[
        VerbDictionaryEntry { lemma: "odložiti", addition: "", transitive: true, imperfective: false },
    ],
    "odlěpiti" => &[
        VerbDictionaryEntry { lemma: "odlěpiti", addition: "", transitive: true, imperfective: false },
    ],
    "odlěpjati" => &[
        VerbDictionaryEntry { lemma: "odlěpjati", addition: "", transitive: true, imperfective: true },
    ],
    "odlětati" => &[
        VerbDictionaryEntry { lemma: "odlětati", addition: "", transitive: true, imperfective: true },
    ],
    "odmesti" => &[
        VerbDictionaryEntry { lemma: "odmesti", addition: "(odmete)", transitive: true, imperfective: false },
    ],
    "odmetati" => &[
        VerbDictionaryEntry { lemma: "odmetati", addition: "", transitive: true, imperfective: true },
    ],
    "odmetnųti" => &[
        VerbDictionaryEntry { lemma: "odmetnųti", addition: "", transitive: true, imperfective: false },
    ],
    "odmstiti" => &[
        VerbDictionaryEntry { lemma: "odmstiti", addition: "", transitive: true, imperfective: false },
    ],
    "odmětati" => &[
        VerbDictionaryEntry { lemma: "odmětati", addition: "", transitive: true, imperfective: true },
    ],
    "odnesti" => &[
        VerbDictionaryEntry { lemma: "odnesti", addition: "", transitive: true, imperfective: false },
    ],
    "odnositi" => &[
        VerbDictionaryEntry { lemma: "odnositi", addition: "", transitive: true, imperfective: true },
    ],
    "odobriti" => &[
        VerbDictionaryEntry { lemma: "odobriti", addition: "", transitive: true, imperfective: false },
    ],
    "odobrjati" => &[
        VerbDictionaryEntry { lemma: "odobrjati", addition: "", transitive: true, imperfective: true },
    ],
    "odolěti" => &[
        VerbDictionaryEntry { lemma: "odolěti", addition: "", transitive: true, imperfective: false },
    ],
    "odolěvati" => &[
        VerbDictionaryEntry { lemma: "odolěvati", addition: "", transitive: true, imperfective: true },
    ],
    "odomašniti" => &[
        VerbDictionaryEntry { lemma: "odomašniti", addition: "", transitive: true, imperfective: false },
    ],
    "odomašnjati" => &[
        VerbDictionaryEntry { lemma: "odomašnjati", addition: "", transitive: true, imperfective: true },
    ],
    "odosobniti" => &[
        VerbDictionaryEntry { lemma: "odosobniti", addition: "", transitive: true, imperfective: false },
    ],
    "odosobnjati" => &[
        VerbDictionaryEntry { lemma: "odosobnjati", addition: "", transitive: true, imperfective: true },
    ],
    "odpadati" => &[
        VerbDictionaryEntry { lemma: "odpadati", addition: "", transitive: true, imperfective: true },
    ],
    "odpasti" => &[
        VerbDictionaryEntry { lemma: "odpasti", addition: "(odpade)", transitive: true, imperfective: false },
    ],
    "odpečatati" => &[
        VerbDictionaryEntry { lemma: "odpečatati", addition: "", transitive: true, imperfective: false },
    ],
    "odpečatyvati" => &[
        VerbDictionaryEntry { lemma: "odpečatyvati", addition: "", transitive: true, imperfective: true },
    ],
    "odphati" => &[
        VerbDictionaryEntry { lemma: "odphati", addition: "", transitive: true, imperfective: false },
    ],
    "odpihati" => &[
        VerbDictionaryEntry { lemma: "odpihati", addition: "", transitive: true, imperfective: true },
    ],
    "odpiliti" => &[
        VerbDictionaryEntry { lemma: "odpiliti", addition: "", transitive: true, imperfective: false },
    ],
    "odpiljati" => &[
        VerbDictionaryEntry { lemma: "odpiljati", addition: "", transitive: true, imperfective: true },
    ],
    "odpinati" => &[
        VerbDictionaryEntry { lemma: "odpinati", addition: "", transitive: true, imperfective: true },
    ],
    "odpisati" => &[
        VerbDictionaryEntry { lemma: "odpisati", addition: "(odpiše)", transitive: true, imperfective: false },
    ],
    "odpisyvati" => &[
        VerbDictionaryEntry { lemma: "odpisyvati", addition: "", transitive: true, imperfective: true },
    ],
    "odplatiti" => &[
        VerbDictionaryEntry { lemma: "odplatiti", addition: "", transitive: true, imperfective: false },
    ],
    "odplaćati" => &[
        VerbDictionaryEntry { lemma: "odplaćati", addition: "", transitive: true, imperfective: true },
    ],
    "odplesti" => &[
        VerbDictionaryEntry { lemma: "odplesti", addition: "(odplete)", transitive: true, imperfective: false },
    ],
    "odpletati" => &[
        VerbDictionaryEntry { lemma: "odpletati", addition: "", transitive: true, imperfective: true },
    ],
    "odpluti" => &[
        VerbDictionaryEntry { lemma: "odpluti", addition: "(odplove)", transitive: true, imperfective: false },
    ],
    "odplyvati" => &[
        VerbDictionaryEntry { lemma: "odplyvati", addition: "", transitive: true, imperfective: true },
    ],
    "odplyvti" => &[
        VerbDictionaryEntry { lemma: "odplyvti", addition: "", transitive: true, imperfective: false },
    ],
    "odpovědati" => &[
        VerbDictionaryEntry { lemma: "odpovědati", addition: "", transitive: true, imperfective: true },
        VerbDictionaryEntry { lemma: "odpovědati", addition: "(+3)", transitive: true, imperfective: true },
    ],
    "odpověděti" => &[
        VerbDictionaryEntry { lemma: "odpověděti", addition: "(odpově)", transitive: true, imperfective: false },
    ],
    "odpočivati" => &[
        VerbDictionaryEntry { lemma: "odpočivati", addition: "", transitive: true, imperfective: true },
    ],
    "odpočęti" => &[
        VerbDictionaryEntry { lemma: "odpočęti", addition: "(odpočne)", transitive: true, imperfective: false },
    ],
    "odpraviti" => &[
        VerbDictionaryEntry { lemma: "odpraviti", addition: "", transitive: true, imperfective: false },
    ],
    "odpravjati" => &[
        VerbDictionaryEntry { lemma: "odpravjati", addition: "", transitive: true, imperfective: true },
    ],
    "odprašati" => &[
        VerbDictionaryEntry { lemma: "odprašati", addition: "", transitive: true, imperfective: true },
    ],
    "odpråšiti" => &[
        VerbDictionaryEntry { lemma: "odpråšiti", addition: "", transitive: true, imperfective: false },
    ],
    "odpustiti" => &[
        VerbDictionaryEntry { lemma: "odpustiti", addition: "", transitive: true, imperfective: false },
    ],
    "odpušćati" => &[
        VerbDictionaryEntry { lemma: "odpušćati", addition: "", transitive: true, imperfective: true },
    ],
    "odpęti" => &[
        VerbDictionaryEntry { lemma: "odpęti", addition: "(odpne)", transitive: true, imperfective: false },
    ],
    "odpųditi" => &[
        VerbDictionaryEntry { lemma: "odpųditi", addition: "", transitive: true, imperfective: false },
    ],
    "odraditi" => &[
        VerbDictionaryEntry { lemma: "odraditi", addition: "", transitive: true, imperfective: false },
    ],
    "odraziti" => &[
        VerbDictionaryEntry { lemma: "odraziti", addition: "", transitive: true, imperfective: false },
    ],
    "odračati" => &[
        VerbDictionaryEntry { lemma: "odračati", addition: "", transitive: true, imperfective: true },
    ],
    "odrađati" => &[
        VerbDictionaryEntry { lemma: "odrađati", addition: "", transitive: true, imperfective: true },
    ],
    "odražati" => &[
        VerbDictionaryEntry { lemma: "odražati", addition: "", transitive: true, imperfective: true },
    ],
    "odrestavrovati" => &[
        VerbDictionaryEntry { lemma: "odrestavrovati", addition: "", transitive: true, imperfective: false },
    ],
    "odročiti" => &[
        VerbDictionaryEntry { lemma: "odročiti", addition: "", transitive: true, imperfective: false },
    ],
    "odrvati" => &[
        VerbDictionaryEntry { lemma: "odrvati", addition: "(odrve)", transitive: true, imperfective: false },
    ],
    "odryti" => &[
        VerbDictionaryEntry { lemma: "odryti", addition: "", transitive: true, imperfective: false },
    ],
    "odryvati" => &[
        VerbDictionaryEntry { lemma: "odryvati", addition: "", transitive: true, imperfective: true },
    ],
    "odrěkati" => &[
        VerbDictionaryEntry { lemma: "odrěkati", addition: "", transitive: true, imperfective: true },
    ],
    "odrěkti" => &[
        VerbDictionaryEntry { lemma: "odrěkti", addition: "", transitive: true, imperfective: false },
    ],
    "odrězati" => &[
        VerbDictionaryEntry { lemma: "odrězati", addition: "(odrěže)", transitive: true, imperfective: false },
    ],
    "odrězyvati" => &[
        VerbDictionaryEntry { lemma: "odrězyvati", addition: "", transitive: true, imperfective: true },
    ],
    "odrųbati" => &[
        VerbDictionaryEntry { lemma: "odrųbati", addition: "", transitive: true, imperfective: false },
    ],
    "odrųbyvati" => &[
        VerbDictionaryEntry { lemma: "odrųbyvati", addition: "", transitive: true, imperfective: true },
    ],
    "odskakati" => &[
        VerbDictionaryEntry { lemma: "odskakati", addition: "", transitive: true, imperfective: true },
    ],
    "odskočiti" => &[
        VerbDictionaryEntry { lemma: "odskočiti", addition: "", transitive: true, imperfective: false },
    ],
    "odslanjati" => &[
        VerbDictionaryEntry { lemma: "odslanjati", addition: "", transitive: true, imperfective: true },
    ],
    "odslati" => &[
        VerbDictionaryEntry { lemma: "odslati", addition: "(odšlje)", transitive: true, imperfective: false },
    ],
    "odsloniti" => &[
        VerbDictionaryEntry { lemma: "odsloniti", addition: "", transitive: true, imperfective: false },
    ],
    "odslužiti" => &[
        VerbDictionaryEntry { lemma: "odslužiti", addition: "", transitive: true, imperfective: false },
    ],
    "odstati" => &[
        VerbDictionaryEntry { lemma: "odstati", addition: "(odstane)", transitive: true, imperfective: false },
    ],
    "odstavati" => &[
        VerbDictionaryEntry { lemma: "odstavati", addition: "", transitive: true, imperfective: true },
    ],
    "odstaviti" => &[
        VerbDictionaryEntry { lemma: "odstaviti", addition: "", transitive: true, imperfective: false },
    ],
    "odstavjati" => &[
        VerbDictionaryEntry { lemma: "odstavjati", addition: "", transitive: true, imperfective: true },
    ],
    "odstranjati" => &[
        VerbDictionaryEntry { lemma: "odstranjati", addition: "", transitive: true, imperfective: true },
    ],
    "odstrašati" => &[
        VerbDictionaryEntry { lemma: "odstrašati", addition: "", transitive: true, imperfective: true },
    ],
    "odstrašiti" => &[
        VerbDictionaryEntry { lemma: "odstrašiti", addition: "", transitive: true, imperfective: false },
    ],
    "odstrigati" => &[
        VerbDictionaryEntry { lemma: "odstrigati", addition: "", transitive: true, imperfective: true },
    ],
    "odstrigti" => &[
        VerbDictionaryEntry { lemma: "odstrigti", addition: "", transitive: true, imperfective: false },
    ],
    "odstråniti" => &[
        VerbDictionaryEntry { lemma: "odstråniti", addition: "", transitive: true, imperfective: false },
    ],
    "odstrěliti" => &[
        VerbDictionaryEntry { lemma: "odstrěliti", addition: "", transitive: true, imperfective: false },
    ],
    "odstųpati" => &[
        VerbDictionaryEntry { lemma: "odstųpati", addition: "", transitive: true, imperfective: true },
    ],
    "odstųpiti" => &[
        VerbDictionaryEntry { lemma: "odstųpiti", addition: "", transitive: true, imperfective: false },
    ],
    "odsunųti" => &[
        VerbDictionaryEntry { lemma: "odsunųti", addition: "", transitive: true, imperfective: false },
    ],
    "odsuvati" => &[
        VerbDictionaryEntry { lemma: "odsuvati", addition: "", transitive: true, imperfective: true },
    ],
    "odsyhati" => &[
        VerbDictionaryEntry { lemma: "odsyhati", addition: "", transitive: true, imperfective: true },
    ],
    "odsylati" => &[
        VerbDictionaryEntry { lemma: "odsylati", addition: "", transitive: true, imperfective: true },
    ],
    "odsěděti" => &[
        VerbDictionaryEntry { lemma: "odsěděti", addition: "", transitive: true, imperfective: false },
    ],
    "odsěkati" => &[
        VerbDictionaryEntry { lemma: "odsěkati", addition: "", transitive: true, imperfective: true },
    ],
    "odsěkti" => &[
        VerbDictionaryEntry { lemma: "odsěkti", addition: "", transitive: true, imperfective: false },
    ],
    "odsěđati" => &[
        VerbDictionaryEntry { lemma: "odsěđati", addition: "", transitive: true, imperfective: true },
    ],
    "odsųtstvovati" => &[
        VerbDictionaryEntry { lemma: "odsųtstvovati", addition: "", transitive: true, imperfective: true },
    ],
    "odsȯhnųti" => &[
        VerbDictionaryEntry { lemma: "odsȯhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "odtajati" => &[
        VerbDictionaryEntry { lemma: "odtajati", addition: "", transitive: true, imperfective: false },
    ],
    "odtekti" => &[
        VerbDictionaryEntry { lemma: "odtekti", addition: "", transitive: true, imperfective: false },
    ],
    "odtirati" => &[
        VerbDictionaryEntry { lemma: "odtirati", addition: "", transitive: true, imperfective: true },
    ],
    "odtiskati" => &[
        VerbDictionaryEntry { lemma: "odtiskati", addition: "", transitive: true, imperfective: true },
    ],
    "odtisknųti" => &[
        VerbDictionaryEntry { lemma: "odtisknųti", addition: "", transitive: true, imperfective: false },
    ],
    "odtrěti" => &[
        VerbDictionaryEntry { lemma: "odtrěti", addition: "(odtre)", transitive: true, imperfective: false },
    ],
    "odtęgati" => &[
        VerbDictionaryEntry { lemma: "odtęgati", addition: "", transitive: true, imperfective: true },
    ],
    "odtęgnųti" => &[
        VerbDictionaryEntry { lemma: "odtęgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "odtěkati" => &[
        VerbDictionaryEntry { lemma: "odtěkati", addition: "", transitive: true, imperfective: true },
    ],
    "odtŕti" => &[
        VerbDictionaryEntry { lemma: "odtŕti", addition: "(odtre)", transitive: true, imperfective: false },
    ],
    "odurěti" => &[
        VerbDictionaryEntry { lemma: "odurěti", addition: "", transitive: true, imperfective: false },
    ],
    "odučati" => &[
        VerbDictionaryEntry { lemma: "odučati", addition: "", transitive: true, imperfective: true },
    ],
    "odučiti" => &[
        VerbDictionaryEntry { lemma: "odučiti", addition: "", transitive: true, imperfective: false },
    ],
    "odvadnjati" => &[
        VerbDictionaryEntry { lemma: "odvadnjati", addition: "", transitive: true, imperfective: true },
    ],
    "odvesti" => &[
        VerbDictionaryEntry { lemma: "odvesti", addition: "(odvede)", transitive: true, imperfective: false },
    ],
    "odvezti" => &[
        VerbDictionaryEntry { lemma: "odvezti", addition: "", transitive: true, imperfective: false },
    ],
    "odvinųti" => &[
        VerbDictionaryEntry { lemma: "odvinųti", addition: "", transitive: true, imperfective: false },
    ],
    "odvivati" => &[
        VerbDictionaryEntry { lemma: "odvivati", addition: "", transitive: true, imperfective: true },
    ],
    "odvlåčivati" => &[
        VerbDictionaryEntry { lemma: "odvlåčivati", addition: "", transitive: true, imperfective: true },
    ],
    "odvlěkati" => &[
        VerbDictionaryEntry { lemma: "odvlěkati", addition: "", transitive: true, imperfective: true },
    ],
    "odvlěkti" => &[
        VerbDictionaryEntry { lemma: "odvlěkti", addition: "", transitive: true, imperfective: false },
    ],
    "odvoditi" => &[
        VerbDictionaryEntry { lemma: "odvoditi", addition: "", transitive: true, imperfective: true },
    ],
    "odvodniti" => &[
        VerbDictionaryEntry { lemma: "odvodniti", addition: "", transitive: true, imperfective: false },
    ],
    "odvoziti" => &[
        VerbDictionaryEntry { lemma: "odvoziti", addition: "", transitive: true, imperfective: true },
    ],
    "odvraćati" => &[
        VerbDictionaryEntry { lemma: "odvraćati", addition: "", transitive: true, imperfective: true },
    ],
    "odvråtiti" => &[
        VerbDictionaryEntry { lemma: "odvråtiti", addition: "", transitive: true, imperfective: false },
    ],
    "odvykati" => &[
        VerbDictionaryEntry { lemma: "odvykati", addition: "", transitive: true, imperfective: true },
    ],
    "odvyknųti" => &[
        VerbDictionaryEntry { lemma: "odvyknųti", addition: "", transitive: true, imperfective: false },
    ],
    "odvęzati" => &[
        VerbDictionaryEntry { lemma: "odvęzati", addition: "(odvęže)", transitive: true, imperfective: false },
    ],
    "odvęzyvati" => &[
        VerbDictionaryEntry { lemma: "odvęzyvati", addition: "", transitive: true, imperfective: true },
    ],
    "odvŕgati" => &[
        VerbDictionaryEntry { lemma: "odvŕgati", addition: "", transitive: true, imperfective: true },
    ],
    "odvŕgnųti" => &[
        VerbDictionaryEntry { lemma: "odvŕgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "odzavisiti" => &[
        VerbDictionaryEntry { lemma: "odzavisiti", addition: "", transitive: true, imperfective: false },
    ],
    "odzvati" => &[
        VerbDictionaryEntry { lemma: "odzvati", addition: "(odzȯve)", transitive: true, imperfective: false },
    ],
    "odzyvati" => &[
        VerbDictionaryEntry { lemma: "odzyvati", addition: "", transitive: true, imperfective: true },
    ],
    "odćuđati" => &[
        VerbDictionaryEntry { lemma: "odćuđati", addition: "", transitive: true, imperfective: true },
    ],
    "odćuđiti" => &[
        VerbDictionaryEntry { lemma: "odćuđiti", addition: "", transitive: true, imperfective: false },
    ],
    "odčajati" => &[
        VerbDictionaryEntry { lemma: "odčajati", addition: "(odčaje)", transitive: true, imperfective: false },
    ],
    "odčajivati" => &[
        VerbDictionaryEntry { lemma: "odčajivati", addition: "", transitive: true, imperfective: true },
    ],
    "odčiniti" => &[
        VerbDictionaryEntry { lemma: "odčiniti", addition: "", transitive: true, imperfective: false },
    ],
    "odčinjati" => &[
        VerbDictionaryEntry { lemma: "odčinjati", addition: "", transitive: true, imperfective: true },
    ],
    "odčitati" => &[
        VerbDictionaryEntry { lemma: "odčitati", addition: "", transitive: true, imperfective: false },
    ],
    "odčityvati" => &[
        VerbDictionaryEntry { lemma: "odčityvati", addition: "", transitive: true, imperfective: true },
    ],
    "odčuvati" => &[
        VerbDictionaryEntry { lemma: "odčuvati", addition: "", transitive: true, imperfective: true },
    ],
    "oděti" => &[
        VerbDictionaryEntry { lemma: "oděti", addition: "(oděne)", transitive: true, imperfective: false },
    ],
    "oděvati" => &[
        VerbDictionaryEntry { lemma: "oděvati", addition: "", transitive: true, imperfective: true },
    ],
    "odšlupati" => &[
        VerbDictionaryEntry { lemma: "odšlupati", addition: "", transitive: true, imperfective: false },
    ],
    "odšlupyvati" => &[
        VerbDictionaryEntry { lemma: "odšlupyvati", addition: "", transitive: true, imperfective: true },
    ],
    "odščepiti" => &[
        VerbDictionaryEntry { lemma: "odščepiti", addition: "", transitive: true, imperfective: false },
    ],
    "odščepjati" => &[
        VerbDictionaryEntry { lemma: "odščepjati", addition: "", transitive: true, imperfective: true },
    ],
    "ogladiti" => &[
        VerbDictionaryEntry { lemma: "ogladiti", addition: "", transitive: true, imperfective: false },
    ],
    "oglašati" => &[
        VerbDictionaryEntry { lemma: "oglašati", addition: "", transitive: true, imperfective: true },
    ],
    "ogluhnųti" => &[
        VerbDictionaryEntry { lemma: "ogluhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "oglupěti" => &[
        VerbDictionaryEntry { lemma: "oglupěti", addition: "", transitive: true, imperfective: false },
    ],
    "oglušeti" => &[
        VerbDictionaryEntry { lemma: "oglušeti", addition: "", transitive: true, imperfective: false },
    ],
    "oglušiti" => &[
        VerbDictionaryEntry { lemma: "oglušiti", addition: "", transitive: true, imperfective: false },
    ],
    "oglåsiti" => &[
        VerbDictionaryEntry { lemma: "oglåsiti", addition: "", transitive: true, imperfective: false },
    ],
    "oględati" => &[
        VerbDictionaryEntry { lemma: "oględati", addition: "", transitive: true, imperfective: true },
    ],
    "oględěti" => &[
        VerbDictionaryEntry { lemma: "oględěti", addition: "(oględi)", transitive: true, imperfective: false },
    ],
    "ognojiti" => &[
        VerbDictionaryEntry { lemma: "ognojiti", addition: "", transitive: true, imperfective: false },
    ],
    "ogoliti" => &[
        VerbDictionaryEntry { lemma: "ogoliti", addition: "", transitive: true, imperfective: false },
    ],
    "ogorčati" => &[
        VerbDictionaryEntry { lemma: "ogorčati", addition: "", transitive: true, imperfective: true },
    ],
    "ogorčiti" => &[
        VerbDictionaryEntry { lemma: "ogorčiti", addition: "", transitive: true, imperfective: false },
    ],
    "ogovarjati" => &[
        VerbDictionaryEntry { lemma: "ogovarjati", addition: "", transitive: true, imperfective: true },
    ],
    "ogovoriti" => &[
        VerbDictionaryEntry { lemma: "ogovoriti", addition: "", transitive: true, imperfective: false },
    ],
    "ograbiti" => &[
        VerbDictionaryEntry { lemma: "ograbiti", addition: "", transitive: true, imperfective: false },
    ],
    "ograbjati" => &[
        VerbDictionaryEntry { lemma: "ograbjati", addition: "", transitive: true, imperfective: true },
    ],
    "ograničati" => &[
        VerbDictionaryEntry { lemma: "ograničati", addition: "", transitive: true, imperfective: true },
    ],
    "ograničiti" => &[
        VerbDictionaryEntry { lemma: "ograničiti", addition: "", transitive: true, imperfective: false },
    ],
    "ograđati" => &[
        VerbDictionaryEntry { lemma: "ograđati", addition: "", transitive: true, imperfective: true },
    ],
    "ogražati" => &[
        VerbDictionaryEntry { lemma: "ogražati", addition: "", transitive: true, imperfective: true },
    ],
    "ogroziti" => &[
        VerbDictionaryEntry { lemma: "ogroziti", addition: "", transitive: true, imperfective: false },
    ],
    "ogråditi" => &[
        VerbDictionaryEntry { lemma: "ogråditi", addition: "", transitive: true, imperfective: false },
    ],
    "ogrěti" => &[
        VerbDictionaryEntry { lemma: "ogrěti", addition: "", transitive: true, imperfective: false },
    ],
    "ogrěvati" => &[
        VerbDictionaryEntry { lemma: "ogrěvati", addition: "", transitive: true, imperfective: true },
    ],
    "ohati" => &[
        VerbDictionaryEntry { lemma: "ohati", addition: "", transitive: true, imperfective: true },
    ],
    "ohlađati" => &[
        VerbDictionaryEntry { lemma: "ohlađati", addition: "", transitive: true, imperfective: true },
    ],
    "ohlåditi" => &[
        VerbDictionaryEntry { lemma: "ohlåditi", addition: "", transitive: true, imperfective: false },
    ],
    "ohlåděti" => &[
        VerbDictionaryEntry { lemma: "ohlåděti", addition: "", transitive: true, imperfective: false },
    ],
    "ohranjati" => &[
        VerbDictionaryEntry { lemma: "ohranjati", addition: "", transitive: true, imperfective: true },
    ],
    "ohroměti" => &[
        VerbDictionaryEntry { lemma: "ohroměti", addition: "", transitive: true, imperfective: false },
    ],
    "ohråniti" => &[
        VerbDictionaryEntry { lemma: "ohråniti", addition: "", transitive: true, imperfective: false },
    ],
    "okameniti" => &[
        VerbDictionaryEntry { lemma: "okameniti", addition: "", transitive: true, imperfective: false },
    ],
    "okamenjati" => &[
        VerbDictionaryEntry { lemma: "okamenjati", addition: "", transitive: true, imperfective: true },
    ],
    "okameněti" => &[
        VerbDictionaryEntry { lemma: "okameněti", addition: "", transitive: true, imperfective: false },
    ],
    "okazati" => &[
        VerbDictionaryEntry { lemma: "okazati", addition: "(okaže)", transitive: true, imperfective: false },
    ],
    "okazyvati" => &[
        VerbDictionaryEntry { lemma: "okazyvati", addition: "", transitive: true, imperfective: true },
    ],
    "oklevetati" => &[
        VerbDictionaryEntry { lemma: "oklevetati", addition: "", transitive: true, imperfective: false },
    ],
    "okolorovati" => &[
        VerbDictionaryEntry { lemma: "okolorovati", addition: "", transitive: true, imperfective: false },
    ],
    "okovati" => &[
        VerbDictionaryEntry { lemma: "okovati", addition: "", transitive: true, imperfective: false },
    ],
    "okropiti" => &[
        VerbDictionaryEntry { lemma: "okropiti", addition: "(+5)", transitive: true, imperfective: false },
    ],
    "okropjati" => &[
        VerbDictionaryEntry { lemma: "okropjati", addition: "(+5)", transitive: true, imperfective: true },
    ],
    "okrėstiti" => &[
        VerbDictionaryEntry { lemma: "okrėstiti", addition: "", transitive: true, imperfective: false },
    ],
    "okrěpnųti" => &[
        VerbDictionaryEntry { lemma: "okrěpnųti", addition: "", transitive: true, imperfective: false },
    ],
    "okrųgliti" => &[
        VerbDictionaryEntry { lemma: "okrųgliti", addition: "", transitive: true, imperfective: false },
    ],
    "okrųgljati" => &[
        VerbDictionaryEntry { lemma: "okrųgljati", addition: "", transitive: true, imperfective: true },
    ],
    "okrųžati" => &[
        VerbDictionaryEntry { lemma: "okrųžati", addition: "", transitive: true, imperfective: true },
    ],
    "okrųžiti" => &[
        VerbDictionaryEntry { lemma: "okrųžiti", addition: "", transitive: true, imperfective: false },
    ],
    "okupovati" => &[
        VerbDictionaryEntry { lemma: "okupovati", addition: "", transitive: true, imperfective: true },
    ],
    "omamiti" => &[
        VerbDictionaryEntry { lemma: "omamiti", addition: "", transitive: true, imperfective: false },
    ],
    "omastiti" => &[
        VerbDictionaryEntry { lemma: "omastiti", addition: "", transitive: true, imperfective: false },
    ],
    "omesti" => &[
        VerbDictionaryEntry { lemma: "omesti", addition: "(omete)", transitive: true, imperfective: false },
    ],
    "omlađati" => &[
        VerbDictionaryEntry { lemma: "omlađati", addition: "", transitive: true, imperfective: true },
    ],
    "omlåditi" => &[
        VerbDictionaryEntry { lemma: "omlåditi", addition: "", transitive: true, imperfective: false },
    ],
    "omlåděti" => &[
        VerbDictionaryEntry { lemma: "omlåděti", addition: "", transitive: true, imperfective: false },
    ],
    "omlěti" => &[
        VerbDictionaryEntry { lemma: "omlěti", addition: "", transitive: true, imperfective: false },
    ],
    "omlěvati" => &[
        VerbDictionaryEntry { lemma: "omlěvati", addition: "", transitive: true, imperfective: true },
    ],
    "omyti" => &[
        VerbDictionaryEntry { lemma: "omyti", addition: "", transitive: true, imperfective: false },
    ],
    "omětati" => &[
        VerbDictionaryEntry { lemma: "omětati", addition: "", transitive: true, imperfective: true },
    ],
    "omŕtvěti" => &[
        VerbDictionaryEntry { lemma: "omŕtvěti", addition: "", transitive: true, imperfective: false },
    ],
    "omŕziti" => &[
        VerbDictionaryEntry { lemma: "omŕziti", addition: "", transitive: true, imperfective: false },
    ],
    "omŕžati" => &[
        VerbDictionaryEntry { lemma: "omŕžati", addition: "", transitive: true, imperfective: true },
    ],
    "omųžiti" => &[
        VerbDictionaryEntry { lemma: "omųžiti", addition: "", transitive: true, imperfective: false },
    ],
    "onesměliti" => &[
        VerbDictionaryEntry { lemma: "onesměliti", addition: "", transitive: true, imperfective: false },
    ],
    "onesměljati" => &[
        VerbDictionaryEntry { lemma: "onesměljati", addition: "", transitive: true, imperfective: true },
    ],
    "oněměti" => &[
        VerbDictionaryEntry { lemma: "oněměti", addition: "", transitive: true, imperfective: false },
    ],
    "opadati" => &[
        VerbDictionaryEntry { lemma: "opadati", addition: "", transitive: true, imperfective: true },
    ],
    "opakovati" => &[
        VerbDictionaryEntry { lemma: "opakovati", addition: "", transitive: true, imperfective: false },
    ],
    "opakovyvati" => &[
        VerbDictionaryEntry { lemma: "opakovyvati", addition: "", transitive: true, imperfective: true },
    ],
    "opaliti" => &[
        VerbDictionaryEntry { lemma: "opaliti", addition: "", transitive: true, imperfective: false },
    ],
    "opaljati" => &[
        VerbDictionaryEntry { lemma: "opaljati", addition: "", transitive: true, imperfective: true },
    ],
    "opariti" => &[
        VerbDictionaryEntry { lemma: "opariti", addition: "", transitive: true, imperfective: false },
    ],
    "oparjati" => &[
        VerbDictionaryEntry { lemma: "oparjati", addition: "", transitive: true, imperfective: true },
    ],
    "opasti" => &[
        VerbDictionaryEntry { lemma: "opasti", addition: "(opade)", transitive: true, imperfective: false },
    ],
    "opekati" => &[
        VerbDictionaryEntry { lemma: "opekati", addition: "", transitive: true, imperfective: true },
    ],
    "opisati" => &[
        VerbDictionaryEntry { lemma: "opisati", addition: "(opiše)", transitive: true, imperfective: false },
    ],
    "opisyvati" => &[
        VerbDictionaryEntry { lemma: "opisyvati", addition: "", transitive: true, imperfective: true },
    ],
    "opiti" => &[
        VerbDictionaryEntry { lemma: "opiti", addition: "(opije)", transitive: true, imperfective: false },
    ],
    "opivati" => &[
        VerbDictionaryEntry { lemma: "opivati", addition: "", transitive: true, imperfective: true },
    ],
    "oplakati" => &[
        VerbDictionaryEntry { lemma: "oplakati", addition: "(oplače)", transitive: true, imperfective: false },
    ],
    "oplakyvati" => &[
        VerbDictionaryEntry { lemma: "oplakyvati", addition: "", transitive: true, imperfective: true },
    ],
    "opljunųti" => &[
        VerbDictionaryEntry { lemma: "opljunųti", addition: "", transitive: true, imperfective: false },
    ],
    "opljuvati" => &[
        VerbDictionaryEntry { lemma: "opljuvati", addition: "", transitive: true, imperfective: true },
    ],
    "oploditi" => &[
        VerbDictionaryEntry { lemma: "oploditi", addition: "", transitive: true, imperfective: false },
    ],
    "oplođati" => &[
        VerbDictionaryEntry { lemma: "oplođati", addition: "", transitive: true, imperfective: true },
    ],
    "opluti" => &[
        VerbDictionaryEntry { lemma: "opluti", addition: "(oplove)", transitive: true, imperfective: false },
    ],
    "oplyvati" => &[
        VerbDictionaryEntry { lemma: "oplyvati", addition: "", transitive: true, imperfective: true },
    ],
    "oplěti" => &[
        VerbDictionaryEntry { lemma: "oplěti", addition: "(oplěje/oplěve)", transitive: true, imperfective: false },
    ],
    "oporędčati" => &[
        VerbDictionaryEntry { lemma: "oporędčati", addition: "", transitive: true, imperfective: true },
    ],
    "oporędčiti" => &[
        VerbDictionaryEntry { lemma: "oporędčiti", addition: "", transitive: true, imperfective: false },
    ],
    "opozdniti" => &[
        VerbDictionaryEntry { lemma: "opozdniti", addition: "", transitive: true, imperfective: false },
    ],
    "opozdnjati" => &[
        VerbDictionaryEntry { lemma: "opozdnjati", addition: "", transitive: true, imperfective: true },
    ],
    "opoznati" => &[
        VerbDictionaryEntry { lemma: "opoznati", addition: "", transitive: true, imperfective: false },
    ],
    "opoznavati" => &[
        VerbDictionaryEntry { lemma: "opoznavati", addition: "", transitive: true, imperfective: true },
    ],
    "opravdati" => &[
        VerbDictionaryEntry { lemma: "opravdati", addition: "", transitive: true, imperfective: false },
    ],
    "opravdyvati" => &[
        VerbDictionaryEntry { lemma: "opravdyvati", addition: "", transitive: true, imperfective: true },
    ],
    "opraviti" => &[
        VerbDictionaryEntry { lemma: "opraviti", addition: "", transitive: true, imperfective: false },
    ],
    "opravjati" => &[
        VerbDictionaryEntry { lemma: "opravjati", addition: "", transitive: true, imperfective: true },
    ],
    "opravniti" => &[
        VerbDictionaryEntry { lemma: "opravniti", addition: "", transitive: true, imperfective: false },
    ],
    "opravnjati" => &[
        VerbDictionaryEntry { lemma: "opravnjati", addition: "", transitive: true, imperfective: true },
    ],
    "oprovŕgati" => &[
        VerbDictionaryEntry { lemma: "oprovŕgati", addition: "", transitive: true, imperfective: true },
    ],
    "oprovŕgnųti" => &[
        VerbDictionaryEntry { lemma: "oprovŕgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "opråzdniti" => &[
        VerbDictionaryEntry { lemma: "opråzdniti", addition: "", transitive: true, imperfective: false },
    ],
    "opråzdnjati" => &[
        VerbDictionaryEntry { lemma: "opråzdnjati", addition: "", transitive: true, imperfective: true },
    ],
    "oprěděliti" => &[
        VerbDictionaryEntry { lemma: "oprěděliti", addition: "", transitive: true, imperfective: false },
    ],
    "oprěděljati" => &[
        VerbDictionaryEntry { lemma: "oprěděljati", addition: "", transitive: true, imperfective: true },
    ],
    "optimizovati" => &[
        VerbDictionaryEntry { lemma: "optimizovati", addition: "", transitive: true, imperfective: true },
    ],
    "opublikovati" => &[
        VerbDictionaryEntry { lemma: "opublikovati", addition: "", transitive: true, imperfective: false },
    ],
    "opuhati" => &[
        VerbDictionaryEntry { lemma: "opuhati", addition: "", transitive: true, imperfective: true },
    ],
    "opuhnųti" => &[
        VerbDictionaryEntry { lemma: "opuhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "opustašati" => &[
        VerbDictionaryEntry { lemma: "opustašati", addition: "", transitive: true, imperfective: true },
    ],
    "opustiti" => &[
        VerbDictionaryEntry { lemma: "opustiti", addition: "", transitive: true, imperfective: false },
    ],
    "opustošiti" => &[
        VerbDictionaryEntry { lemma: "opustošiti", addition: "", transitive: true, imperfective: false },
    ],
    "opustěti" => &[
        VerbDictionaryEntry { lemma: "opustěti", addition: "", transitive: true, imperfective: false },
    ],
    "opušćati" => &[
        VerbDictionaryEntry { lemma: "opušćati", addition: "", transitive: true, imperfective: true },
    ],
    "opyliti" => &[
        VerbDictionaryEntry { lemma: "opyliti", addition: "", transitive: true, imperfective: false },
    ],
    "opyljati" => &[
        VerbDictionaryEntry { lemma: "opyljati", addition: "", transitive: true, imperfective: true },
    ],
    "orati" => &[
        VerbDictionaryEntry { lemma: "orati", addition: "(oŕe)", transitive: false, imperfective: true },
    ],
    "orašati" => &[
        VerbDictionaryEntry { lemma: "orašati", addition: "", transitive: true, imperfective: true },
    ],
    "organizovati" => &[
        VerbDictionaryEntry { lemma: "organizovati", addition: "", transitive: true, imperfective: true },
    ],
    "oriti" => &[
        VerbDictionaryEntry { lemma: "oriti", addition: "", transitive: true, imperfective: true },
    ],
    "orkestrovati" => &[
        VerbDictionaryEntry { lemma: "orkestrovati", addition: "", transitive: true, imperfective: true },
    ],
    "orositi" => &[
        VerbDictionaryEntry { lemma: "orositi", addition: "", transitive: true, imperfective: false },
    ],
    "orųdovati" => &[
        VerbDictionaryEntry { lemma: "orųdovati", addition: "(+5)", transitive: true, imperfective: true },
    ],
    "orųžiti" => &[
        VerbDictionaryEntry { lemma: "orųžiti", addition: "", transitive: true, imperfective: true },
    ],
    "osaditi" => &[
        VerbDictionaryEntry { lemma: "osaditi", addition: "", transitive: true, imperfective: false },
    ],
    "osamotiti" => &[
        VerbDictionaryEntry { lemma: "osamotiti", addition: "", transitive: true, imperfective: false },
    ],
    "osađati" => &[
        VerbDictionaryEntry { lemma: "osađati", addition: "", transitive: true, imperfective: true },
    ],
    "osedlati" => &[
        VerbDictionaryEntry { lemma: "osedlati", addition: "", transitive: true, imperfective: false },
    ],
    "osiliti" => &[
        VerbDictionaryEntry { lemma: "osiliti", addition: "", transitive: true, imperfective: false },
    ],
    "osiljati" => &[
        VerbDictionaryEntry { lemma: "osiljati", addition: "", transitive: true, imperfective: true },
    ],
    "osirotiti" => &[
        VerbDictionaryEntry { lemma: "osirotiti", addition: "", transitive: true, imperfective: false },
    ],
    "osirotěti" => &[
        VerbDictionaryEntry { lemma: "osirotěti", addition: "", transitive: true, imperfective: false },
    ],
    "osivěti" => &[
        VerbDictionaryEntry { lemma: "osivěti", addition: "", transitive: true, imperfective: false },
    ],
    "oskopiti" => &[
        VerbDictionaryEntry { lemma: "oskopiti", addition: "", transitive: true, imperfective: false },
    ],
    "oskubati" => &[
        VerbDictionaryEntry { lemma: "oskubati", addition: "", transitive: true, imperfective: false },
    ],
    "oskubyvati" => &[
        VerbDictionaryEntry { lemma: "oskubyvati", addition: "", transitive: true, imperfective: true },
    ],
    "oskvŕniti" => &[
        VerbDictionaryEntry { lemma: "oskvŕniti", addition: "", transitive: true, imperfective: false },
    ],
    "oskvŕnjati" => &[
        VerbDictionaryEntry { lemma: "oskvŕnjati", addition: "", transitive: true, imperfective: true },
    ],
    "oslabiti" => &[
        VerbDictionaryEntry { lemma: "oslabiti", addition: "", transitive: true, imperfective: false },
    ],
    "oslabjati" => &[
        VerbDictionaryEntry { lemma: "oslabjati", addition: "", transitive: true, imperfective: true },
    ],
    "oslaběti" => &[
        VerbDictionaryEntry { lemma: "oslaběti", addition: "", transitive: true, imperfective: false },
    ],
    "osloženiti" => &[
        VerbDictionaryEntry { lemma: "osloženiti", addition: "", transitive: true, imperfective: false },
    ],
    "osloženjati" => &[
        VerbDictionaryEntry { lemma: "osloženjati", addition: "", transitive: true, imperfective: true },
    ],
    "oslåditi" => &[
        VerbDictionaryEntry { lemma: "oslåditi", addition: "", transitive: true, imperfective: false },
    ],
    "oslěpiti" => &[
        VerbDictionaryEntry { lemma: "oslěpiti", addition: "", transitive: true, imperfective: false },
    ],
    "oslěpjati" => &[
        VerbDictionaryEntry { lemma: "oslěpjati", addition: "", transitive: true, imperfective: true },
    ],
    "oslěpnųti" => &[
        VerbDictionaryEntry { lemma: "oslěpnųti", addition: "", transitive: true, imperfective: false },
    ],
    "osmažiti" => &[
        VerbDictionaryEntry { lemma: "osmažiti", addition: "", transitive: true, imperfective: false },
    ],
    "osmoliti" => &[
        VerbDictionaryEntry { lemma: "osmoliti", addition: "", transitive: true, imperfective: false },
    ],
    "osměliti" => &[
        VerbDictionaryEntry { lemma: "osměliti", addition: "", transitive: true, imperfective: false },
    ],
    "osměljati" => &[
        VerbDictionaryEntry { lemma: "osměljati", addition: "", transitive: true, imperfective: true },
    ],
    "osnovati" => &[
        VerbDictionaryEntry { lemma: "osnovati", addition: "", transitive: true, imperfective: false },
    ],
    "osnovyvati" => &[
        VerbDictionaryEntry { lemma: "osnovyvati", addition: "", transitive: true, imperfective: true },
    ],
    "osněžiti" => &[
        VerbDictionaryEntry { lemma: "osněžiti", addition: "", transitive: true, imperfective: false },
    ],
    "osoliti" => &[
        VerbDictionaryEntry { lemma: "osoliti", addition: "", transitive: true, imperfective: false },
    ],
    "osparjati" => &[
        VerbDictionaryEntry { lemma: "osparjati", addition: "", transitive: true, imperfective: true },
    ],
    "osporiti" => &[
        VerbDictionaryEntry { lemma: "osporiti", addition: "", transitive: true, imperfective: false },
    ],
    "osråmiti" => &[
        VerbDictionaryEntry { lemma: "osråmiti", addition: "", transitive: true, imperfective: false },
    ],
    "ostarěti" => &[
        VerbDictionaryEntry { lemma: "ostarěti", addition: "", transitive: true, imperfective: false },
    ],
    "ostati" => &[
        VerbDictionaryEntry { lemma: "ostati", addition: "(ostane)", transitive: true, imperfective: false },
    ],
    "ostavati" => &[
        VerbDictionaryEntry { lemma: "ostavati", addition: "", transitive: true, imperfective: true },
    ],
    "ostaviti" => &[
        VerbDictionaryEntry { lemma: "ostaviti", addition: "", transitive: true, imperfective: false },
    ],
    "ostavjati" => &[
        VerbDictionaryEntry { lemma: "ostavjati", addition: "", transitive: true, imperfective: true },
    ],
    "ostrigati" => &[
        VerbDictionaryEntry { lemma: "ostrigati", addition: "", transitive: true, imperfective: true },
    ],
    "ostrigti" => &[
        VerbDictionaryEntry { lemma: "ostrigti", addition: "", transitive: true, imperfective: false },
    ],
    "ostriti" => &[
        VerbDictionaryEntry { lemma: "ostriti", addition: "", transitive: true, imperfective: true },
    ],
    "ostrugati" => &[
        VerbDictionaryEntry { lemma: "ostrugati", addition: "", transitive: true, imperfective: false },
    ],
    "ostrěgati" => &[
        VerbDictionaryEntry { lemma: "ostrěgati", addition: "", transitive: true, imperfective: true },
    ],
    "ostrěgti" => &[
        VerbDictionaryEntry { lemma: "ostrěgti", addition: "", transitive: true, imperfective: false },
    ],
    "ostuditi" => &[
        VerbDictionaryEntry { lemma: "ostuditi", addition: "", transitive: true, imperfective: false },
    ],
    "osušati" => &[
        VerbDictionaryEntry { lemma: "osušati", addition: "", transitive: true, imperfective: true },
    ],
    "osušiti" => &[
        VerbDictionaryEntry { lemma: "osušiti", addition: "", transitive: true, imperfective: false },
    ],
    "osvajati" => &[
        VerbDictionaryEntry { lemma: "osvajati", addition: "", transitive: true, imperfective: true },
    ],
    "osvobađati" => &[
        VerbDictionaryEntry { lemma: "osvobađati", addition: "", transitive: true, imperfective: true },
    ],
    "osvoboditi" => &[
        VerbDictionaryEntry { lemma: "osvoboditi", addition: "", transitive: true, imperfective: false },
    ],
    "osvojiti" => &[
        VerbDictionaryEntry { lemma: "osvojiti", addition: "", transitive: true, imperfective: true },
    ],
    "osvętiti" => &[
        VerbDictionaryEntry { lemma: "osvętiti", addition: "", transitive: true, imperfective: false },
    ],
    "osvęćati" => &[
        VerbDictionaryEntry { lemma: "osvęćati", addition: "", transitive: true, imperfective: true },
    ],
    "osvětiti" => &[
        VerbDictionaryEntry { lemma: "osvětiti", addition: "", transitive: true, imperfective: false },
    ],
    "osvětliti" => &[
        VerbDictionaryEntry { lemma: "osvětliti", addition: "", transitive: true, imperfective: false },
    ],
    "osvětljati" => &[
        VerbDictionaryEntry { lemma: "osvětljati", addition: "", transitive: true, imperfective: true },
    ],
    "osvěćati" => &[
        VerbDictionaryEntry { lemma: "osvěćati", addition: "", transitive: true, imperfective: true },
    ],
    "osvěžati" => &[
        VerbDictionaryEntry { lemma: "osvěžati", addition: "", transitive: true, imperfective: true },
    ],
    "osvěžiti" => &[
        VerbDictionaryEntry { lemma: "osvěžiti", addition: "", transitive: true, imperfective: false },
    ],
    "osědati" => &[
        VerbDictionaryEntry { lemma: "osědati", addition: "", transitive: true, imperfective: true },
    ],
    "osěsti" => &[
        VerbDictionaryEntry { lemma: "osěsti", addition: "(osěde)", transitive: true, imperfective: false },
    ],
    "osųditi" => &[
        VerbDictionaryEntry { lemma: "osųditi", addition: "", transitive: true, imperfective: false },
    ],
    "osųđati" => &[
        VerbDictionaryEntry { lemma: "osųđati", addition: "", transitive: true, imperfective: true },
    ],
    "osȯvrěmenniti" => &[
        VerbDictionaryEntry { lemma: "osȯvrěmenniti", addition: "", transitive: true, imperfective: false },
    ],
    "osȯvrěmennjati" => &[
        VerbDictionaryEntry { lemma: "osȯvrěmennjati", addition: "", transitive: true, imperfective: true },
    ],
    "otišati" => &[
        VerbDictionaryEntry { lemma: "otišati", addition: "", transitive: true, imperfective: true },
    ],
    "otišiti" => &[
        VerbDictionaryEntry { lemma: "otišiti", addition: "", transitive: true, imperfective: false },
    ],
    "otraviti" => &[
        VerbDictionaryEntry { lemma: "otraviti", addition: "", transitive: true, imperfective: false },
    ],
    "otravjati" => &[
        VerbDictionaryEntry { lemma: "otravjati", addition: "", transitive: true, imperfective: true },
    ],
    "otrudniti" => &[
        VerbDictionaryEntry { lemma: "otrudniti", addition: "", transitive: true, imperfective: false },
    ],
    "otrudnjati" => &[
        VerbDictionaryEntry { lemma: "otrudnjati", addition: "", transitive: true, imperfective: true },
    ],
    "otręsati" => &[
        VerbDictionaryEntry { lemma: "otręsati", addition: "", transitive: true, imperfective: true },
    ],
    "otręsti" => &[
        VerbDictionaryEntry { lemma: "otręsti", addition: "", transitive: true, imperfective: false },
    ],
    "otrězviti" => &[
        VerbDictionaryEntry { lemma: "otrězviti", addition: "", transitive: true, imperfective: false },
    ],
    "otrězvjati" => &[
        VerbDictionaryEntry { lemma: "otrězvjati", addition: "", transitive: true, imperfective: true },
    ],
    "otrězvěti" => &[
        VerbDictionaryEntry { lemma: "otrězvěti", addition: "", transitive: true, imperfective: false },
    ],
    "otvarjati" => &[
        VerbDictionaryEntry { lemma: "otvarjati", addition: "", transitive: true, imperfective: true },
    ],
    "otvoriti" => &[
        VerbDictionaryEntry { lemma: "otvoriti", addition: "", transitive: true, imperfective: false },
    ],
    "otvŕděti" => &[
        VerbDictionaryEntry { lemma: "otvŕděti", addition: "", transitive: true, imperfective: false },
    ],
    "otęgčati" => &[
        VerbDictionaryEntry { lemma: "otęgčati", addition: "", transitive: true, imperfective: true },
    ],
    "otęgčiti" => &[
        VerbDictionaryEntry { lemma: "otęgčiti", addition: "", transitive: true, imperfective: false },
    ],
    "otěniti" => &[
        VerbDictionaryEntry { lemma: "otěniti", addition: "", transitive: true, imperfective: false },
    ],
    "otěnjati" => &[
        VerbDictionaryEntry { lemma: "otěnjati", addition: "", transitive: true, imperfective: true },
    ],
    "otųpiti" => &[
        VerbDictionaryEntry { lemma: "otųpiti", addition: "", transitive: true, imperfective: false },
    ],
    "otųpjati" => &[
        VerbDictionaryEntry { lemma: "otųpjati", addition: "", transitive: true, imperfective: true },
    ],
    "otųpěti" => &[
        VerbDictionaryEntry { lemma: "otųpěti", addition: "", transitive: true, imperfective: false },
    ],
    "ovdověti" => &[
        VerbDictionaryEntry { lemma: "ovdověti", addition: "", transitive: true, imperfective: false },
    ],
    "ovinųti" => &[
        VerbDictionaryEntry { lemma: "ovinųti", addition: "", transitive: true, imperfective: false },
    ],
    "ovivati" => &[
        VerbDictionaryEntry { lemma: "ovivati", addition: "", transitive: true, imperfective: true },
    ],
    "ovladnųti" => &[
        VerbDictionaryEntry { lemma: "ovladnųti", addition: "", transitive: true, imperfective: false },
    ],
    "ovladyvati" => &[
        VerbDictionaryEntry { lemma: "ovladyvati", addition: "", transitive: true, imperfective: true },
    ],
    "ovplyvniti" => &[
        VerbDictionaryEntry { lemma: "ovplyvniti", addition: "", transitive: true, imperfective: false },
    ],
    "ovplyvnjati" => &[
        VerbDictionaryEntry { lemma: "ovplyvnjati", addition: "", transitive: true, imperfective: true },
    ],
    "ovějati" => &[
        VerbDictionaryEntry { lemma: "ovějati", addition: "(ověje)", transitive: true, imperfective: false },
    ],
    "ovějivati" => &[
        VerbDictionaryEntry { lemma: "ovějivati", addition: "", transitive: true, imperfective: true },
    ],
    "ověnčati" => &[
        VerbDictionaryEntry { lemma: "ověnčati", addition: "", transitive: true, imperfective: true },
    ],
    "ověnčiti" => &[
        VerbDictionaryEntry { lemma: "ověnčiti", addition: "", transitive: true, imperfective: false },
    ],
    "ověriti" => &[
        VerbDictionaryEntry { lemma: "ověriti", addition: "", transitive: true, imperfective: false },
    ],
    "ověrjati" => &[
        VerbDictionaryEntry { lemma: "ověrjati", addition: "", transitive: true, imperfective: true },
    ],
    "ovȯlgnųti" => &[
        VerbDictionaryEntry { lemma: "ovȯlgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "ozdabjati" => &[
        VerbDictionaryEntry { lemma: "ozdabjati", addition: "", transitive: true, imperfective: true },
    ],
    "ozdobiti" => &[
        VerbDictionaryEntry { lemma: "ozdobiti", addition: "", transitive: true, imperfective: false },
    ],
    "ozdravjati" => &[
        VerbDictionaryEntry { lemma: "ozdravjati", addition: "", transitive: true, imperfective: true },
    ],
    "ozdråviti" => &[
        VerbDictionaryEntry { lemma: "ozdråviti", addition: "", transitive: true, imperfective: false },
    ],
    "ozdråvěti" => &[
        VerbDictionaryEntry { lemma: "ozdråvěti", addition: "", transitive: true, imperfective: false },
    ],
    "ozeleniti" => &[
        VerbDictionaryEntry { lemma: "ozeleniti", addition: "", transitive: true, imperfective: false },
    ],
    "ozelenjati" => &[
        VerbDictionaryEntry { lemma: "ozelenjati", addition: "", transitive: true, imperfective: true },
    ],
    "ozlobiti" => &[
        VerbDictionaryEntry { lemma: "ozlobiti", addition: "", transitive: true, imperfective: false },
    ],
    "oznamenovati" => &[
        VerbDictionaryEntry { lemma: "oznamenovati", addition: "", transitive: true, imperfective: false },
    ],
    "oznamenovyvati" => &[
        VerbDictionaryEntry { lemma: "oznamenovyvati", addition: "", transitive: true, imperfective: true },
    ],
    "oznanjati" => &[
        VerbDictionaryEntry { lemma: "oznanjati", addition: "", transitive: true, imperfective: true },
    ],
    "označati" => &[
        VerbDictionaryEntry { lemma: "označati", addition: "", transitive: true, imperfective: true },
    ],
    "označiti" => &[
        VerbDictionaryEntry { lemma: "označiti", addition: "", transitive: true, imperfective: false },
    ],
    "ozvěrěti" => &[
        VerbDictionaryEntry { lemma: "ozvěrěti", addition: "", transitive: true, imperfective: false },
    ],
    "ozębti" => &[
        VerbDictionaryEntry { lemma: "ozębti", addition: "", transitive: true, imperfective: false },
    ],
    "očariti" => &[
        VerbDictionaryEntry { lemma: "očariti", addition: "", transitive: false, imperfective: false },
    ],
    "očarovati" => &[
        VerbDictionaryEntry { lemma: "očarovati", addition: "", transitive: true, imperfective: false },
        VerbDictionaryEntry { lemma: "očarovati", addition: "(+3)", transitive: false, imperfective: false },
    ],
    "očarovyvati" => &[
        VerbDictionaryEntry { lemma: "očarovyvati", addition: "", transitive: true, imperfective: true },
        VerbDictionaryEntry { lemma: "očarovyvati", addition: "(+3)", transitive: false, imperfective: true },
    ],
    "očekyvati" => &[
        VerbDictionaryEntry { lemma: "očekyvati", addition: "", transitive: true, imperfective: true },
    ],
    "očistiti" => &[
        VerbDictionaryEntry { lemma: "očistiti", addition: "", transitive: true, imperfective: false },
    ],
    "očišćati" => &[
        VerbDictionaryEntry { lemma: "očišćati", addition: "", transitive: true, imperfective: true },
    ],
    "očrkati" => &[
        VerbDictionaryEntry { lemma: "očrkati", addition: "", transitive: true, imperfective: true },
    ],
    "očrknųti" => &[
        VerbDictionaryEntry { lemma: "očrknųti", addition: "", transitive: true, imperfective: false },
    ],
    "očrniti" => &[
        VerbDictionaryEntry { lemma: "očrniti", addition: "", transitive: true, imperfective: false },
    ],
    "očrnjati" => &[
        VerbDictionaryEntry { lemma: "očrnjati", addition: "", transitive: true, imperfective: true },
    ],
    "ošalěti" => &[
        VerbDictionaryEntry { lemma: "ošalěti", addition: "", transitive: true, imperfective: false },
    ],
    "oženiti" => &[
        VerbDictionaryEntry { lemma: "oženiti", addition: "", transitive: true, imperfective: false },
    ],
    "ožiti" => &[
        VerbDictionaryEntry { lemma: "ožiti", addition: "(ožive)", transitive: true, imperfective: false },
    ],
    "oživati" => &[
        VerbDictionaryEntry { lemma: "oživati", addition: "", transitive: true, imperfective: true },
    ],
    "oživiti" => &[
        VerbDictionaryEntry { lemma: "oživiti", addition: "", transitive: true, imperfective: false },
    ],
    "oživjati" => &[
        VerbDictionaryEntry { lemma: "oživjati", addition: "", transitive: true, imperfective: true },
    ],
    "ožȯltěti" => &[
        VerbDictionaryEntry { lemma: "ožȯltěti", addition: "", transitive: true, imperfective: false },
    ],
    "padati" => &[
        VerbDictionaryEntry { lemma: "padati", addition: "", transitive: true, imperfective: true },
    ],
    "pakovati" => &[
        VerbDictionaryEntry { lemma: "pakovati", addition: "", transitive: true, imperfective: true },
    ],
    "palatalizovati" => &[
        VerbDictionaryEntry { lemma: "palatalizovati", addition: "", transitive: true, imperfective: true },
    ],
    "paliti" => &[
        VerbDictionaryEntry { lemma: "paliti", addition: "", transitive: true, imperfective: true },
    ],
    "pamętati" => &[
        VerbDictionaryEntry { lemma: "pamętati", addition: "", transitive: true, imperfective: true },
    ],
    "panikovati" => &[
        VerbDictionaryEntry { lemma: "panikovati", addition: "", transitive: true, imperfective: true },
    ],
    "paralizovati" => &[
        VerbDictionaryEntry { lemma: "paralizovati", addition: "", transitive: true, imperfective: true },
    ],
    "parazitovati" => &[
        VerbDictionaryEntry { lemma: "parazitovati", addition: "", transitive: true, imperfective: true },
    ],
    "parfumovati" => &[
        VerbDictionaryEntry { lemma: "parfumovati", addition: "", transitive: true, imperfective: true },
    ],
    "pariti" => &[
        VerbDictionaryEntry { lemma: "pariti", addition: "", transitive: true, imperfective: true },
    ],
    "parkovati" => &[
        VerbDictionaryEntry { lemma: "parkovati", addition: "", transitive: true, imperfective: true },
    ],
    "pasti" => &[
        VerbDictionaryEntry { lemma: "pasti", addition: "(pade)", transitive: true, imperfective: false },
        VerbDictionaryEntry { lemma: "pasti", addition: "(pase)", transitive: true, imperfective: true },
    ],
    "patronizovati" => &[
        VerbDictionaryEntry { lemma: "patronizovati", addition: "", transitive: true, imperfective: true },
    ],
    "pauzovati" => &[
        VerbDictionaryEntry { lemma: "pauzovati", addition: "", transitive: true, imperfective: true },
    ],
    "pekti" => &[
        VerbDictionaryEntry { lemma: "pekti", addition: "", transitive: true, imperfective: true },
    ],
    "pečatati" => &[
        VerbDictionaryEntry { lemma: "pečatati", addition: "", transitive: true, imperfective: true },
    ],
    "phati" => &[
        VerbDictionaryEntry { lemma: "phati", addition: "", transitive: true, imperfective: true },
    ],
    "phnųti" => &[
        VerbDictionaryEntry { lemma: "phnųti", addition: "", transitive: true, imperfective: false },
    ],
    "pikirovati" => &[
        VerbDictionaryEntry { lemma: "pikirovati", addition: "", transitive: true, imperfective: true },
    ],
    "piliti" => &[
        VerbDictionaryEntry { lemma: "piliti", addition: "", transitive: true, imperfective: true },
    ],
    "pinati" => &[
        VerbDictionaryEntry { lemma: "pinati", addition: "", transitive: true, imperfective: true },
    ],
    "pirovati" => &[
        VerbDictionaryEntry { lemma: "pirovati", addition: "", transitive: true, imperfective: true },
    ],
    "pisati" => &[
        VerbDictionaryEntry { lemma: "pisati", addition: "(piše)", transitive: true, imperfective: true },
    ],
    "piskati" => &[
        VerbDictionaryEntry { lemma: "piskati", addition: "", transitive: true, imperfective: true },
    ],
    "piti" => &[
        VerbDictionaryEntry { lemma: "piti", addition: "(pije)", transitive: true, imperfective: true },
    ],
    "pišati" => &[
        VerbDictionaryEntry { lemma: "pišati", addition: "", transitive: true, imperfective: true },
    ],
    "piščeti" => &[
        VerbDictionaryEntry { lemma: "piščeti", addition: "(pišče)", transitive: true, imperfective: true },
    ],
    "plakati" => &[
        VerbDictionaryEntry { lemma: "plakati", addition: "(plače)", transitive: true, imperfective: true },
    ],
    "planovati" => &[
        VerbDictionaryEntry { lemma: "planovati", addition: "", transitive: true, imperfective: true },
    ],
    "platiti" => &[
        VerbDictionaryEntry { lemma: "platiti", addition: "", transitive: true, imperfective: true },
    ],
    "plavati" => &[
        VerbDictionaryEntry { lemma: "plavati", addition: "", transitive: true, imperfective: true },
    ],
    "pleskati" => &[
        VerbDictionaryEntry { lemma: "pleskati", addition: "", transitive: true, imperfective: true },
    ],
    "plesknųti" => &[
        VerbDictionaryEntry { lemma: "plesknųti", addition: "", transitive: true, imperfective: false },
    ],
    "plesti" => &[
        VerbDictionaryEntry { lemma: "plesti", addition: "(plete)", transitive: true, imperfective: true },
    ],
    "pljunųti" => &[
        VerbDictionaryEntry { lemma: "pljunųti", addition: "", transitive: true, imperfective: false },
    ],
    "pljuskati" => &[
        VerbDictionaryEntry { lemma: "pljuskati", addition: "", transitive: true, imperfective: true },
    ],
    "pljusknųti" => &[
        VerbDictionaryEntry { lemma: "pljusknųti", addition: "", transitive: true, imperfective: false },
    ],
    "pljuvati" => &[
        VerbDictionaryEntry { lemma: "pljuvati", addition: "", transitive: true, imperfective: true },
    ],
    "ploditi" => &[
        VerbDictionaryEntry { lemma: "ploditi", addition: "", transitive: true, imperfective: true },
    ],
    "pluti" => &[
        VerbDictionaryEntry { lemma: "pluti", addition: "(plove)", transitive: true, imperfective: true },
    ],
    "plyvti" => &[
        VerbDictionaryEntry { lemma: "plyvti", addition: "", transitive: true, imperfective: true },
    ],
    "plęsati" => &[
        VerbDictionaryEntry { lemma: "plęsati", addition: "", transitive: true, imperfective: true },
    ],
    "plěniti" => &[
        VerbDictionaryEntry { lemma: "plěniti", addition: "", transitive: true, imperfective: true },
    ],
    "plěti" => &[
        VerbDictionaryEntry { lemma: "plěti", addition: "(plěje/plěve)", transitive: true, imperfective: true },
    ],
    "pobiti" => &[
        VerbDictionaryEntry { lemma: "pobiti", addition: "(pobije)", transitive: true, imperfective: false },
    ],
    "poblågodariti" => &[
        VerbDictionaryEntry { lemma: "poblågodariti", addition: "", transitive: true, imperfective: false },
    ],
    "poblågoželati" => &[
        VerbDictionaryEntry { lemma: "poblågoželati", addition: "(+3)", transitive: true, imperfective: false },
    ],
    "pobuditi" => &[
        VerbDictionaryEntry { lemma: "pobuditi", addition: "", transitive: true, imperfective: false },
    ],
    "pobuđati" => &[
        VerbDictionaryEntry { lemma: "pobuđati", addition: "", transitive: true, imperfective: true },
    ],
    "poběditi" => &[
        VerbDictionaryEntry { lemma: "poběditi", addition: "", transitive: true, imperfective: false },
    ],
    "poběgti" => &[
        VerbDictionaryEntry { lemma: "poběgti", addition: "(poběži)", transitive: true, imperfective: false },
    ],
    "poběđati" => &[
        VerbDictionaryEntry { lemma: "poběđati", addition: "", transitive: true, imperfective: true },
    ],
    "pocělovati" => &[
        VerbDictionaryEntry { lemma: "pocělovati", addition: "", transitive: true, imperfective: false },
    ],
    "podariti" => &[
        VerbDictionaryEntry { lemma: "podariti", addition: "", transitive: true, imperfective: false },
    ],
    "podati" => &[
        VerbDictionaryEntry { lemma: "podati", addition: "(poda)", transitive: true, imperfective: false },
    ],
    "podavati" => &[
        VerbDictionaryEntry { lemma: "podavati", addition: "", transitive: true, imperfective: true },
    ],
    "podbirati" => &[
        VerbDictionaryEntry { lemma: "podbirati", addition: "", transitive: true, imperfective: true },
    ],
    "podbrati" => &[
        VerbDictionaryEntry { lemma: "podbrati", addition: "(podbere)", transitive: true, imperfective: false },
    ],
    "poddati" => &[
        VerbDictionaryEntry { lemma: "poddati", addition: "(podda)", transitive: true, imperfective: false },
    ],
    "poddavati" => &[
        VerbDictionaryEntry { lemma: "poddavati", addition: "", transitive: true, imperfective: true },
    ],
    "poddŕžati" => &[
        VerbDictionaryEntry { lemma: "poddŕžati", addition: "", transitive: true, imperfective: false },
    ],
    "poddŕživati" => &[
        VerbDictionaryEntry { lemma: "poddŕživati", addition: "", transitive: true, imperfective: true },
    ],
    "podględati" => &[
        VerbDictionaryEntry { lemma: "podględati", addition: "", transitive: true, imperfective: true },
    ],
    "podględěti" => &[
        VerbDictionaryEntry { lemma: "podględěti", addition: "(podględi)", transitive: true, imperfective: false },
    ],
    "podgovarjati" => &[
        VerbDictionaryEntry { lemma: "podgovarjati", addition: "", transitive: true, imperfective: true },
    ],
    "podgovoriti" => &[
        VerbDictionaryEntry { lemma: "podgovoriti", addition: "", transitive: true, imperfective: false },
    ],
    "podgrěti" => &[
        VerbDictionaryEntry { lemma: "podgrěti", addition: "(podgrěje)", transitive: true, imperfective: false },
    ],
    "podgrěvati" => &[
        VerbDictionaryEntry { lemma: "podgrěvati", addition: "", transitive: true, imperfective: true },
    ],
    "podhoditi" => &[
        VerbDictionaryEntry { lemma: "podhoditi", addition: "", transitive: true, imperfective: true },
    ],
    "podimati" => &[
        VerbDictionaryEntry { lemma: "podimati", addition: "", transitive: true, imperfective: true },
    ],
    "podjęti" => &[
        VerbDictionaryEntry { lemma: "podjęti", addition: "(podȯjme)", transitive: true, imperfective: false },
    ],
    "podkladati" => &[
        VerbDictionaryEntry { lemma: "podkladati", addition: "", transitive: true, imperfective: true },
    ],
    "podključati" => &[
        VerbDictionaryEntry { lemma: "podključati", addition: "", transitive: true, imperfective: true },
    ],
    "podključiti" => &[
        VerbDictionaryEntry { lemma: "podključiti", addition: "", transitive: true, imperfective: false },
    ],
    "podkopati" => &[
        VerbDictionaryEntry { lemma: "podkopati", addition: "", transitive: true, imperfective: false },
    ],
    "podkopyvati" => &[
        VerbDictionaryEntry { lemma: "podkopyvati", addition: "", transitive: true, imperfective: true },
    ],
    "podkovati" => &[
        VerbDictionaryEntry { lemma: "podkovati", addition: "", transitive: true, imperfective: false },
    ],
    "podkovyvati" => &[
        VerbDictionaryEntry { lemma: "podkovyvati", addition: "", transitive: true, imperfective: true },
    ],
    "podkrěpiti" => &[
        VerbDictionaryEntry { lemma: "podkrěpiti", addition: "", transitive: true, imperfective: false },
    ],
    "podkrěpjati" => &[
        VerbDictionaryEntry { lemma: "podkrěpjati", addition: "", transitive: true, imperfective: true },
    ],
    "podkupiti" => &[
        VerbDictionaryEntry { lemma: "podkupiti", addition: "", transitive: true, imperfective: false },
    ],
    "podkupovati" => &[
        VerbDictionaryEntry { lemma: "podkupovati", addition: "", transitive: true, imperfective: true },
    ],
    "podlagati" => &[
        VerbDictionaryEntry { lemma: "podlagati", addition: "", transitive: true, imperfective: true },
    ],
    "podležati" => &[
        VerbDictionaryEntry { lemma: "podležati", addition: "(podleži) (+3)", transitive: true, imperfective: true },
    ],
    "podliti" => &[
        VerbDictionaryEntry { lemma: "podliti", addition: "(podlije)", transitive: true, imperfective: false },
    ],
    "podlivati" => &[
        VerbDictionaryEntry { lemma: "podlivati", addition: "", transitive: true, imperfective: true },
    ],
    "podložiti" => &[
        VerbDictionaryEntry { lemma: "podložiti", addition: "", transitive: true, imperfective: false },
    ],
    "podměniti" => &[
        VerbDictionaryEntry { lemma: "podměniti", addition: "", transitive: true, imperfective: false },
    ],
    "podměnjati" => &[
        VerbDictionaryEntry { lemma: "podměnjati", addition: "", transitive: true, imperfective: true },
    ],
    "podnurjati" => &[
        VerbDictionaryEntry { lemma: "podnurjati", addition: "", transitive: true, imperfective: true },
    ],
    "podpaliti" => &[
        VerbDictionaryEntry { lemma: "podpaliti", addition: "", transitive: true, imperfective: false },
    ],
    "podpaljati" => &[
        VerbDictionaryEntry { lemma: "podpaljati", addition: "", transitive: true, imperfective: true },
    ],
    "podpirati" => &[
        VerbDictionaryEntry { lemma: "podpirati", addition: "", transitive: true, imperfective: true },
    ],
    "podpisati" => &[
        VerbDictionaryEntry { lemma: "podpisati", addition: "(podpiše)", transitive: true, imperfective: false },
    ],
    "podpisyvati" => &[
        VerbDictionaryEntry { lemma: "podpisyvati", addition: "", transitive: true, imperfective: true },
    ],
    "podporiti" => &[
        VerbDictionaryEntry { lemma: "podporiti", addition: "", transitive: true, imperfective: false },
    ],
    "podråzděliti" => &[
        VerbDictionaryEntry { lemma: "podråzděliti", addition: "", transitive: true, imperfective: false },
    ],
    "podråzděljati" => &[
        VerbDictionaryEntry { lemma: "podråzděljati", addition: "", transitive: true, imperfective: true },
    ],
    "podręditi" => &[
        VerbDictionaryEntry { lemma: "podręditi", addition: "", transitive: true, imperfective: false },
    ],
    "podręđati" => &[
        VerbDictionaryEntry { lemma: "podręđati", addition: "", transitive: true, imperfective: true },
    ],
    "podslušati" => &[
        VerbDictionaryEntry { lemma: "podslušati", addition: "", transitive: true, imperfective: false },
    ],
    "podslušivati" => &[
        VerbDictionaryEntry { lemma: "podslušivati", addition: "", transitive: true, imperfective: true },
    ],
    "podstrěkati" => &[
        VerbDictionaryEntry { lemma: "podstrěkati", addition: "(podstrěče)", transitive: true, imperfective: true },
    ],
    "podstrěknųti" => &[
        VerbDictionaryEntry { lemma: "podstrěknųti", addition: "", transitive: true, imperfective: false },
    ],
    "podtiskati" => &[
        VerbDictionaryEntry { lemma: "podtiskati", addition: "", transitive: true, imperfective: true },
    ],
    "podtisknųti" => &[
        VerbDictionaryEntry { lemma: "podtisknųti", addition: "", transitive: true, imperfective: false },
    ],
    "podtrimati" => &[
        VerbDictionaryEntry { lemma: "podtrimati", addition: "", transitive: true, imperfective: false },
    ],
    "podtrimyvati" => &[
        VerbDictionaryEntry { lemma: "podtrimyvati", addition: "", transitive: true, imperfective: true },
    ],
    "podvajati" => &[
        VerbDictionaryEntry { lemma: "podvajati", addition: "", transitive: true, imperfective: true },
    ],
    "podvažati" => &[
        VerbDictionaryEntry { lemma: "podvažati", addition: "", transitive: true, imperfective: true },
    ],
    "podvažiti" => &[
        VerbDictionaryEntry { lemma: "podvažiti", addition: "", transitive: true, imperfective: false },
    ],
    "podvisiti" => &[
        VerbDictionaryEntry { lemma: "podvisiti", addition: "", transitive: true, imperfective: false },
    ],
    "podvojiti" => &[
        VerbDictionaryEntry { lemma: "podvojiti", addition: "", transitive: true, imperfective: true },
    ],
    "podvŕgati" => &[
        VerbDictionaryEntry { lemma: "podvŕgati", addition: "", transitive: true, imperfective: true },
    ],
    "podvŕgnųti" => &[
        VerbDictionaryEntry { lemma: "podvŕgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "podzirati" => &[
        VerbDictionaryEntry { lemma: "podzirati", addition: "", transitive: true, imperfective: true },
    ],
    "podzrěti" => &[
        VerbDictionaryEntry { lemma: "podzrěti", addition: "(podzri)", transitive: true, imperfective: false },
    ],
    "podzrěvati" => &[
        VerbDictionaryEntry { lemma: "podzrěvati", addition: "", transitive: true, imperfective: true },
    ],
    "podčrkati" => &[
        VerbDictionaryEntry { lemma: "podčrkati", addition: "", transitive: true, imperfective: true },
    ],
    "podčrknųti" => &[
        VerbDictionaryEntry { lemma: "podčrknųti", addition: "", transitive: true, imperfective: false },
    ],
    "podękovati" => &[
        VerbDictionaryEntry { lemma: "podękovati", addition: "(+3)", transitive: true, imperfective: false },
    ],
    "poděliti" => &[
        VerbDictionaryEntry { lemma: "poděliti", addition: "", transitive: true, imperfective: false },
    ],
    "podȯjdti" => &[
        VerbDictionaryEntry { lemma: "podȯjdti", addition: "(podȯjde; podšėl)", transitive: true, imperfective: false },
    ],
    "pogaršati" => &[
        VerbDictionaryEntry { lemma: "pogaršati", addition: "", transitive: true, imperfective: true },
    ],
    "pogladiti" => &[
        VerbDictionaryEntry { lemma: "pogladiti", addition: "", transitive: true, imperfective: false },
    ],
    "poględati" => &[
        VerbDictionaryEntry { lemma: "poględati", addition: "", transitive: true, imperfective: false },
    ],
    "poględnųti" => &[
        VerbDictionaryEntry { lemma: "poględnųti", addition: "", transitive: true, imperfective: false },
    ],
    "poględyvati" => &[
        VerbDictionaryEntry { lemma: "poględyvati", addition: "", transitive: true, imperfective: true },
    ],
    "poglųbiti" => &[
        VerbDictionaryEntry { lemma: "poglųbiti", addition: "", transitive: true, imperfective: false },
    ],
    "poglųbjati" => &[
        VerbDictionaryEntry { lemma: "poglųbjati", addition: "", transitive: true, imperfective: true },
    ],
    "pogoršiti" => &[
        VerbDictionaryEntry { lemma: "pogoršiti", addition: "", transitive: true, imperfective: false },
    ],
    "pogovoriti" => &[
        VerbDictionaryEntry { lemma: "pogovoriti", addition: "", transitive: true, imperfective: false },
    ],
    "pogrebati" => &[
        VerbDictionaryEntry { lemma: "pogrebati", addition: "", transitive: true, imperfective: true },
    ],
    "pogrebti" => &[
        VerbDictionaryEntry { lemma: "pogrebti", addition: "", transitive: true, imperfective: false },
    ],
    "pogrěšiti" => &[
        VerbDictionaryEntry { lemma: "pogrěšiti", addition: "", transitive: true, imperfective: false },
    ],
    "pogrųziti" => &[
        VerbDictionaryEntry { lemma: "pogrųziti", addition: "", transitive: true, imperfective: false },
    ],
    "pogrųžati" => &[
        VerbDictionaryEntry { lemma: "pogrųžati", addition: "", transitive: true, imperfective: true },
    ],
    "pogynati" => &[
        VerbDictionaryEntry { lemma: "pogynati", addition: "", transitive: true, imperfective: true },
    ],
    "pogynųti" => &[
        VerbDictionaryEntry { lemma: "pogynųti", addition: "", transitive: true, imperfective: false },
    ],
    "pogȯltnųti" => &[
        VerbDictionaryEntry { lemma: "pogȯltnųti", addition: "", transitive: true, imperfective: false },
    ],
    "pohristijaniti" => &[
        VerbDictionaryEntry { lemma: "pohristijaniti", addition: "", transitive: true, imperfective: false },
    ],
    "pohristijanjati" => &[
        VerbDictionaryEntry { lemma: "pohristijanjati", addition: "", transitive: true, imperfective: true },
    ],
    "pohvaliti" => &[
        VerbDictionaryEntry { lemma: "pohvaliti", addition: "", transitive: true, imperfective: false },
    ],
    "pohytiti" => &[
        VerbDictionaryEntry { lemma: "pohytiti", addition: "", transitive: true, imperfective: false },
    ],
    "pohyćati" => &[
        VerbDictionaryEntry { lemma: "pohyćati", addition: "", transitive: true, imperfective: true },
    ],
    "poiskati" => &[
        VerbDictionaryEntry { lemma: "poiskati", addition: "(poišče)", transitive: true, imperfective: false },
    ],
    "poiskyvati" => &[
        VerbDictionaryEntry { lemma: "poiskyvati", addition: "", transitive: true, imperfective: true },
    ],
    "pojdti" => &[
        VerbDictionaryEntry { lemma: "pojdti", addition: "(pojde; pošėl)", transitive: true, imperfective: false },
    ],
    "pojehati" => &[
        VerbDictionaryEntry { lemma: "pojehati", addition: "(pojede)", transitive: true, imperfective: false },
    ],
    "pojiti" => &[
        VerbDictionaryEntry { lemma: "pojiti", addition: "", transitive: true, imperfective: true },
    ],
    "pojmati" => &[
        VerbDictionaryEntry { lemma: "pojmati", addition: "", transitive: true, imperfective: true },
    ],
    "pojęti" => &[
        VerbDictionaryEntry { lemma: "pojęti", addition: "(pojme)", transitive: true, imperfective: false },
    ],
    "pokarati" => &[
        VerbDictionaryEntry { lemma: "pokarati", addition: "(pokare)", transitive: true, imperfective: false },
        VerbDictionaryEntry { lemma: "pokarati", addition: "", transitive: true, imperfective: false },
    ],
    "pokarjati" => &[
        VerbDictionaryEntry { lemma: "pokarjati", addition: "", transitive: true, imperfective: true },
    ],
    "pokazati" => &[
        VerbDictionaryEntry { lemma: "pokazati", addition: "(pokaže)", transitive: true, imperfective: false },
    ],
    "pokazyvati" => &[
        VerbDictionaryEntry { lemma: "pokazyvati", addition: "", transitive: true, imperfective: true },
    ],
    "pokladati" => &[
        VerbDictionaryEntry { lemma: "pokladati", addition: "", transitive: true, imperfective: true },
    ],
    "poklicati" => &[
        VerbDictionaryEntry { lemma: "poklicati", addition: "", transitive: true, imperfective: true },
    ],
    "pokoriti" => &[
        VerbDictionaryEntry { lemma: "pokoriti", addition: "", transitive: true, imperfective: false },
    ],
    "pokositi" => &[
        VerbDictionaryEntry { lemma: "pokositi", addition: "", transitive: true, imperfective: false },
    ],
    "pokryti" => &[
        VerbDictionaryEntry { lemma: "pokryti", addition: "", transitive: true, imperfective: false },
    ],
    "pokryvati" => &[
        VerbDictionaryEntry { lemma: "pokryvati", addition: "", transitive: true, imperfective: true },
    ],
    "pokrėstiti" => &[
        VerbDictionaryEntry { lemma: "pokrėstiti", addition: "", transitive: true, imperfective: false },
    ],
    "pokusiti" => &[
        VerbDictionaryEntry { lemma: "pokusiti", addition: "", transitive: true, imperfective: false },
    ],
    "pokušati" => &[
        VerbDictionaryEntry { lemma: "pokušati", addition: "", transitive: true, imperfective: true },
    ],
    "polakovati" => &[
        VerbDictionaryEntry { lemma: "polakovati", addition: "", transitive: true, imperfective: false },
    ],
    "poletěti" => &[
        VerbDictionaryEntry { lemma: "poletěti", addition: "(poleti)", transitive: true, imperfective: false },
    ],
    "polirati" => &[
        VerbDictionaryEntry { lemma: "polirati", addition: "", transitive: true, imperfective: true },
    ],
    "politi" => &[
        VerbDictionaryEntry { lemma: "politi", addition: "(polije)", transitive: true, imperfective: false },
    ],
    "polivati" => &[
        VerbDictionaryEntry { lemma: "polivati", addition: "", transitive: true, imperfective: true },
    ],
    "poljzovati" => &[
        VerbDictionaryEntry { lemma: "poljzovati", addition: "", transitive: true, imperfective: true },
    ],
    "položiti" => &[
        VerbDictionaryEntry { lemma: "položiti", addition: "", transitive: true, imperfective: false },
    ],
    "polučati" => &[
        VerbDictionaryEntry { lemma: "polučati", addition: "", transitive: true, imperfective: true },
    ],
    "polučiti" => &[
        VerbDictionaryEntry { lemma: "polučiti", addition: "", transitive: true, imperfective: false },
    ],
    "polězti" => &[
        VerbDictionaryEntry { lemma: "polězti", addition: "", transitive: true, imperfective: false },
    ],
    "pomagati" => &[
        VerbDictionaryEntry { lemma: "pomagati", addition: "(+3)", transitive: true, imperfective: true },
    ],
    "pomazati" => &[
        VerbDictionaryEntry { lemma: "pomazati", addition: "(pomaže)", transitive: true, imperfective: false },
    ],
    "pomilovati" => &[
        VerbDictionaryEntry { lemma: "pomilovati", addition: "", transitive: true, imperfective: false },
    ],
    "pomiriti" => &[
        VerbDictionaryEntry { lemma: "pomiriti", addition: "", transitive: true, imperfective: false },
    ],
    "pomněti" => &[
        VerbDictionaryEntry { lemma: "pomněti", addition: "(pomni)", transitive: true, imperfective: true },
    ],
    "pomogti" => &[
        VerbDictionaryEntry { lemma: "pomogti", addition: "(+3)", transitive: true, imperfective: false },
    ],
    "pomyliti" => &[
        VerbDictionaryEntry { lemma: "pomyliti", addition: "", transitive: true, imperfective: false },
    ],
    "pomysliti" => &[
        VerbDictionaryEntry { lemma: "pomysliti", addition: "", transitive: true, imperfective: false },
    ],
    "poměstiti" => &[
        VerbDictionaryEntry { lemma: "poměstiti", addition: "", transitive: true, imperfective: false },
    ],
    "poměšćati" => &[
        VerbDictionaryEntry { lemma: "poměšćati", addition: "", transitive: true, imperfective: true },
    ],
    "poniziti" => &[
        VerbDictionaryEntry { lemma: "poniziti", addition: "", transitive: true, imperfective: false },
    ],
    "ponižati" => &[
        VerbDictionaryEntry { lemma: "ponižati", addition: "", transitive: true, imperfective: true },
    ],
    "popiti" => &[
        VerbDictionaryEntry { lemma: "popiti", addition: "(popije)", transitive: true, imperfective: false },
    ],
    "popivati" => &[
        VerbDictionaryEntry { lemma: "popivati", addition: "", transitive: true, imperfective: true },
    ],
    "poplaviti" => &[
        VerbDictionaryEntry { lemma: "poplaviti", addition: "", transitive: true, imperfective: false },
    ],
    "poplavjati" => &[
        VerbDictionaryEntry { lemma: "poplavjati", addition: "", transitive: true, imperfective: true },
    ],
    "popluti" => &[
        VerbDictionaryEntry { lemma: "popluti", addition: "(poplove)", transitive: true, imperfective: false },
    ],
    "poplyvti" => &[
        VerbDictionaryEntry { lemma: "poplyvti", addition: "", transitive: true, imperfective: false },
    ],
    "popraviti" => &[
        VerbDictionaryEntry { lemma: "popraviti", addition: "", transitive: true, imperfective: false },
    ],
    "popravjati" => &[
        VerbDictionaryEntry { lemma: "popravjati", addition: "", transitive: true, imperfective: true },
    ],
    "poprobovati" => &[
        VerbDictionaryEntry { lemma: "poprobovati", addition: "", transitive: true, imperfective: false },
    ],
    "poprositi" => &[
        VerbDictionaryEntry { lemma: "poprositi", addition: "", transitive: true, imperfective: false },
    ],
    "popularizovati" => &[
        VerbDictionaryEntry { lemma: "popularizovati", addition: "", transitive: true, imperfective: true },
    ],
    "popustiti" => &[
        VerbDictionaryEntry { lemma: "popustiti", addition: "", transitive: true, imperfective: false },
    ],
    "popušćati" => &[
        VerbDictionaryEntry { lemma: "popušćati", addition: "", transitive: true, imperfective: true },
    ],
    "popȯlzati" => &[
        VerbDictionaryEntry { lemma: "popȯlzati", addition: "", transitive: true, imperfective: true },
    ],
    "popȯlzti" => &[
        VerbDictionaryEntry { lemma: "popȯlzti", addition: "", transitive: true, imperfective: false },
    ],
    "poraditi" => &[
        VerbDictionaryEntry { lemma: "poraditi", addition: "", transitive: true, imperfective: false },
    ],
    "poraniti" => &[
        VerbDictionaryEntry { lemma: "poraniti", addition: "", transitive: true, imperfective: false },
    ],
    "poraziti" => &[
        VerbDictionaryEntry { lemma: "poraziti", addition: "", transitive: true, imperfective: false },
    ],
    "porađati" => &[
        VerbDictionaryEntry { lemma: "porađati", addition: "", transitive: true, imperfective: true },
    ],
    "poražati" => &[
        VerbDictionaryEntry { lemma: "poražati", addition: "", transitive: true, imperfective: true },
    ],
    "poroditi" => &[
        VerbDictionaryEntry { lemma: "poroditi", addition: "", transitive: true, imperfective: false },
    ],
    "porušiti" => &[
        VerbDictionaryEntry { lemma: "porušiti", addition: "", transitive: true, imperfective: false },
    ],
    "poråbiti" => &[
        VerbDictionaryEntry { lemma: "poråbiti", addition: "", transitive: true, imperfective: false },
    ],
    "poråbovati" => &[
        VerbDictionaryEntry { lemma: "poråbovati", addition: "", transitive: true, imperfective: true },
    ],
    "poråsti" => &[
        VerbDictionaryEntry { lemma: "poråsti", addition: "(poråste)", transitive: true, imperfective: false },
    ],
    "poråvniti" => &[
        VerbDictionaryEntry { lemma: "poråvniti", addition: "", transitive: true, imperfective: false },
    ],
    "porųbati" => &[
        VerbDictionaryEntry { lemma: "porųbati", addition: "", transitive: true, imperfective: false },
    ],
    "poslati" => &[
        VerbDictionaryEntry { lemma: "poslati", addition: "(pošlje)", transitive: true, imperfective: false },
    ],
    "poslušati" => &[
        VerbDictionaryEntry { lemma: "poslušati", addition: "", transitive: true, imperfective: false },
    ],
    "posoliti" => &[
        VerbDictionaryEntry { lemma: "posoliti", addition: "", transitive: true, imperfective: false },
    ],
    "pospati" => &[
        VerbDictionaryEntry { lemma: "pospati", addition: "(pospi)", transitive: true, imperfective: false },
    ],
    "pospěšiti" => &[
        VerbDictionaryEntry { lemma: "pospěšiti", addition: "", transitive: true, imperfective: false },
    ],
    "posrěbriti" => &[
        VerbDictionaryEntry { lemma: "posrěbriti", addition: "", transitive: true, imperfective: false },
    ],
    "posrěbrjati" => &[
        VerbDictionaryEntry { lemma: "posrěbrjati", addition: "", transitive: true, imperfective: true },
    ],
    "posrědkovati" => &[
        VerbDictionaryEntry { lemma: "posrědkovati", addition: "", transitive: true, imperfective: true },
    ],
    "postanavjati" => &[
        VerbDictionaryEntry { lemma: "postanavjati", addition: "", transitive: true, imperfective: true },
    ],
    "postanoviti" => &[
        VerbDictionaryEntry { lemma: "postanoviti", addition: "", transitive: true, imperfective: false },
    ],
    "postarěti" => &[
        VerbDictionaryEntry { lemma: "postarěti", addition: "", transitive: true, imperfective: false },
    ],
    "postaviti" => &[
        VerbDictionaryEntry { lemma: "postaviti", addition: "", transitive: true, imperfective: false },
    ],
    "postavjati" => &[
        VerbDictionaryEntry { lemma: "postavjati", addition: "", transitive: true, imperfective: true },
    ],
    "posteliti" => &[
        VerbDictionaryEntry { lemma: "posteliti", addition: "", transitive: true, imperfective: false },
    ],
    "postigati" => &[
        VerbDictionaryEntry { lemma: "postigati", addition: "", transitive: true, imperfective: true },
    ],
    "postignųti" => &[
        VerbDictionaryEntry { lemma: "postignųti", addition: "", transitive: true, imperfective: false },
    ],
    "postlati" => &[
        VerbDictionaryEntry { lemma: "postlati", addition: "(postelje)", transitive: true, imperfective: false },
    ],
    "postulovati" => &[
        VerbDictionaryEntry { lemma: "postulovati", addition: "", transitive: true, imperfective: true },
    ],
    "postųpati" => &[
        VerbDictionaryEntry { lemma: "postųpati", addition: "", transitive: true, imperfective: true },
    ],
    "postųpiti" => &[
        VerbDictionaryEntry { lemma: "postųpiti", addition: "", transitive: true, imperfective: false },
    ],
    "posvętiti" => &[
        VerbDictionaryEntry { lemma: "posvętiti", addition: "", transitive: true, imperfective: false },
    ],
    "posvęćati" => &[
        VerbDictionaryEntry { lemma: "posvęćati", addition: "", transitive: true, imperfective: true },
    ],
    "posvědčati" => &[
        VerbDictionaryEntry { lemma: "posvědčati", addition: "", transitive: true, imperfective: true },
    ],
    "posvědčiti" => &[
        VerbDictionaryEntry { lemma: "posvědčiti", addition: "", transitive: true, imperfective: false },
    ],
    "posylati" => &[
        VerbDictionaryEntry { lemma: "posylati", addition: "", transitive: true, imperfective: true },
    ],
    "posědati" => &[
        VerbDictionaryEntry { lemma: "posědati", addition: "", transitive: true, imperfective: true },
    ],
    "posějati" => &[
        VerbDictionaryEntry { lemma: "posějati", addition: "(posěje)", transitive: true, imperfective: false },
    ],
    "posěkati" => &[
        VerbDictionaryEntry { lemma: "posěkati", addition: "", transitive: true, imperfective: false },
    ],
    "posěkti" => &[
        VerbDictionaryEntry { lemma: "posěkti", addition: "", transitive: true, imperfective: false },
    ],
    "posěsti" => &[
        VerbDictionaryEntry { lemma: "posěsti", addition: "(posěde)", transitive: true, imperfective: false },
    ],
    "posětiti" => &[
        VerbDictionaryEntry { lemma: "posětiti", addition: "", transitive: true, imperfective: false },
    ],
    "posěćati" => &[
        VerbDictionaryEntry { lemma: "posěćati", addition: "", transitive: true, imperfective: true },
    ],
    "posȯvětovati" => &[
        VerbDictionaryEntry { lemma: "posȯvětovati", addition: "", transitive: true, imperfective: false },
    ],
    "potapjati" => &[
        VerbDictionaryEntry { lemma: "potapjati", addition: "", transitive: true, imperfective: true },
    ],
    "potelefonovati" => &[
        VerbDictionaryEntry { lemma: "potelefonovati", addition: "", transitive: true, imperfective: false },
    ],
    "potopiti" => &[
        VerbDictionaryEntry { lemma: "potopiti", addition: "", transitive: true, imperfective: false },
    ],
    "potrajati" => &[
        VerbDictionaryEntry { lemma: "potrajati", addition: "", transitive: true, imperfective: false },
    ],
    "potrvati" => &[
        VerbDictionaryEntry { lemma: "potrvati", addition: "", transitive: true, imperfective: false },
    ],
    "potręsati" => &[
        VerbDictionaryEntry { lemma: "potręsati", addition: "", transitive: true, imperfective: true },
    ],
    "potręsti" => &[
        VerbDictionaryEntry { lemma: "potręsti", addition: "", transitive: true, imperfective: false },
    ],
    "potrěbovati" => &[
        VerbDictionaryEntry { lemma: "potrěbovati", addition: "", transitive: true, imperfective: true },
    ],
    "potvŕditi" => &[
        VerbDictionaryEntry { lemma: "potvŕditi", addition: "", transitive: true, imperfective: false },
    ],
    "potvŕđati" => &[
        VerbDictionaryEntry { lemma: "potvŕđati", addition: "", transitive: true, imperfective: true },
    ],
    "potėmněti" => &[
        VerbDictionaryEntry { lemma: "potėmněti", addition: "", transitive: true, imperfective: false },
    ],
    "potěti" => &[
        VerbDictionaryEntry { lemma: "potěti", addition: "(poti)", transitive: true, imperfective: true },
    ],
    "poučati" => &[
        VerbDictionaryEntry { lemma: "poučati", addition: "", transitive: true, imperfective: true },
    ],
    "poučiti" => &[
        VerbDictionaryEntry { lemma: "poučiti", addition: "", transitive: true, imperfective: false },
    ],
    "povaliti" => &[
        VerbDictionaryEntry { lemma: "povaliti", addition: "", transitive: true, imperfective: false },
    ],
    "považati" => &[
        VerbDictionaryEntry { lemma: "považati", addition: "", transitive: true, imperfective: true },
    ],
    "považiti" => &[
        VerbDictionaryEntry { lemma: "považiti", addition: "", transitive: true, imperfective: false },
    ],
    "povraćati" => &[
        VerbDictionaryEntry { lemma: "povraćati", addition: "", transitive: false, imperfective: true },
    ],
    "povråtiti" => &[
        VerbDictionaryEntry { lemma: "povråtiti", addition: "", transitive: false, imperfective: false },
    ],
    "povstati" => &[
        VerbDictionaryEntry { lemma: "povstati", addition: "(povstane)", transitive: true, imperfective: false },
    ],
    "povstavati" => &[
        VerbDictionaryEntry { lemma: "povstavati", addition: "", transitive: true, imperfective: true },
    ],
    "povtarjati" => &[
        VerbDictionaryEntry { lemma: "povtarjati", addition: "", transitive: true, imperfective: true },
    ],
    "povtoriti" => &[
        VerbDictionaryEntry { lemma: "povtoriti", addition: "", transitive: true, imperfective: false },
    ],
    "povysiti" => &[
        VerbDictionaryEntry { lemma: "povysiti", addition: "", transitive: true, imperfective: false },
    ],
    "povyšati" => &[
        VerbDictionaryEntry { lemma: "povyšati", addition: "", transitive: true, imperfective: true },
    ],
    "povęzati" => &[
        VerbDictionaryEntry { lemma: "povęzati", addition: "", transitive: true, imperfective: false },
    ],
    "povęzyvati" => &[
        VerbDictionaryEntry { lemma: "povęzyvati", addition: "", transitive: true, imperfective: true },
    ],
    "povědati" => &[
        VerbDictionaryEntry { lemma: "povědati", addition: "", transitive: true, imperfective: true },
    ],
    "pověděti" => &[
        VerbDictionaryEntry { lemma: "pověděti", addition: "(pově)", transitive: true, imperfective: false },
    ],
    "pověriti" => &[
        VerbDictionaryEntry { lemma: "pověriti", addition: "", transitive: true, imperfective: false },
    ],
    "pověrjati" => &[
        VerbDictionaryEntry { lemma: "pověrjati", addition: "", transitive: true, imperfective: true },
    ],
    "pověsiti" => &[
        VerbDictionaryEntry { lemma: "pověsiti", addition: "", transitive: true, imperfective: false },
    ],
    "povŕnųti" => &[
        VerbDictionaryEntry { lemma: "povŕnųti", addition: "", transitive: true, imperfective: false },
    ],
    "povŕtati" => &[
        VerbDictionaryEntry { lemma: "povŕtati", addition: "", transitive: true, imperfective: true },
    ],
    "pozajmati" => &[
        VerbDictionaryEntry { lemma: "pozajmati", addition: "", transitive: true, imperfective: true },
    ],
    "pozajęti" => &[
        VerbDictionaryEntry { lemma: "pozajęti", addition: "(pozajme)", transitive: true, imperfective: false },
    ],
    "pozastaviti" => &[
        VerbDictionaryEntry { lemma: "pozastaviti", addition: "", transitive: true, imperfective: false },
    ],
    "pozdravjati" => &[
        VerbDictionaryEntry { lemma: "pozdravjati", addition: "", transitive: true, imperfective: true },
    ],
    "pozdråviti" => &[
        VerbDictionaryEntry { lemma: "pozdråviti", addition: "", transitive: true, imperfective: false },
    ],
    "pozlaćati" => &[
        VerbDictionaryEntry { lemma: "pozlaćati", addition: "", transitive: true, imperfective: true },
    ],
    "pozlåtiti" => &[
        VerbDictionaryEntry { lemma: "pozlåtiti", addition: "", transitive: true, imperfective: false },
    ],
    "poznati" => &[
        VerbDictionaryEntry { lemma: "poznati", addition: "", transitive: true, imperfective: false },
    ],
    "poznavati" => &[
        VerbDictionaryEntry { lemma: "poznavati", addition: "", transitive: true, imperfective: true },
    ],
    "pozovati" => &[
        VerbDictionaryEntry { lemma: "pozovati", addition: "", transitive: true, imperfective: true },
    ],
    "pozvaljati" => &[
        VerbDictionaryEntry { lemma: "pozvaljati", addition: "", transitive: true, imperfective: true },
    ],
    "pozvati" => &[
        VerbDictionaryEntry { lemma: "pozvati", addition: "(pozȯve)", transitive: true, imperfective: false },
    ],
    "pozvoliti" => &[
        VerbDictionaryEntry { lemma: "pozvoliti", addition: "", transitive: true, imperfective: false },
    ],
    "pozvoniti" => &[
        VerbDictionaryEntry { lemma: "pozvoniti", addition: "", transitive: false, imperfective: false },
    ],
    "počekati" => &[
        VerbDictionaryEntry { lemma: "počekati", addition: "", transitive: true, imperfective: true },
    ],
    "počinati" => &[
        VerbDictionaryEntry { lemma: "počinati", addition: "", transitive: true, imperfective: true },
    ],
    "počrveněti" => &[
        VerbDictionaryEntry { lemma: "počrveněti", addition: "", transitive: true, imperfective: false },
    ],
    "počęti" => &[
        VerbDictionaryEntry { lemma: "počęti", addition: "(počne)", transitive: true, imperfective: false },
    ],
    "poškoditi" => &[
        VerbDictionaryEntry { lemma: "poškoditi", addition: "", transitive: true, imperfective: false },
    ],
    "poščęditi" => &[
        VerbDictionaryEntry { lemma: "poščęditi", addition: "", transitive: true, imperfective: false },
    ],
    "požaliti" => &[
        VerbDictionaryEntry { lemma: "požaliti", addition: "", transitive: true, imperfective: false },
    ],
    "poželati" => &[
        VerbDictionaryEntry { lemma: "poželati", addition: "", transitive: true, imperfective: false },
    ],
    "požirati" => &[
        VerbDictionaryEntry { lemma: "požirati", addition: "", transitive: false, imperfective: true },
    ],
    "požrtvovati" => &[
        VerbDictionaryEntry { lemma: "požrtvovati", addition: "", transitive: true, imperfective: false },
    ],
    "požrěti" => &[
        VerbDictionaryEntry { lemma: "požrěti", addition: "", transitive: true, imperfective: false },
    ],
    "požędati" => &[
        VerbDictionaryEntry { lemma: "požędati", addition: "", transitive: true, imperfective: false },
    ],
    "požęti" => &[
        VerbDictionaryEntry { lemma: "požęti", addition: "(požne)", transitive: true, imperfective: false },
    ],
    "pracovati" => &[
        VerbDictionaryEntry { lemma: "pracovati", addition: "", transitive: true, imperfective: true },
    ],
    "praktikovati" => &[
        VerbDictionaryEntry { lemma: "praktikovati", addition: "", transitive: true, imperfective: true },
    ],
    "prati" => &[
        VerbDictionaryEntry { lemma: "prati", addition: "(pere)", transitive: true, imperfective: true },
    ],
    "praviti" => &[
        VerbDictionaryEntry { lemma: "praviti", addition: "", transitive: true, imperfective: true },
    ],
    "prašćati" => &[
        VerbDictionaryEntry { lemma: "prašćati", addition: "", transitive: true, imperfective: true },
    ],
    "preferovati" => &[
        VerbDictionaryEntry { lemma: "preferovati", addition: "", transitive: true, imperfective: true },
    ],
    "pribiti" => &[
        VerbDictionaryEntry { lemma: "pribiti", addition: "(pribije)", transitive: true, imperfective: false },
    ],
    "pribivati" => &[
        VerbDictionaryEntry { lemma: "pribivati", addition: "", transitive: true, imperfective: true },
    ],
    "pribyti" => &[
        VerbDictionaryEntry { lemma: "pribyti", addition: "(pribųde)", transitive: true, imperfective: false },
    ],
    "pribyvati" => &[
        VerbDictionaryEntry { lemma: "pribyvati", addition: "", transitive: true, imperfective: true },
    ],
    "pricěliti" => &[
        VerbDictionaryEntry { lemma: "pricěliti", addition: "", transitive: false, imperfective: false },
    ],
    "pridati" => &[
        VerbDictionaryEntry { lemma: "pridati", addition: "(prida)", transitive: true, imperfective: false },
    ],
    "pridavati" => &[
        VerbDictionaryEntry { lemma: "pridavati", addition: "", transitive: true, imperfective: true },
    ],
    "pridumati" => &[
        VerbDictionaryEntry { lemma: "pridumati", addition: "", transitive: true, imperfective: false },
    ],
    "pridělati" => &[
        VerbDictionaryEntry { lemma: "pridělati", addition: "", transitive: true, imperfective: false },
    ],
    "priděliti" => &[
        VerbDictionaryEntry { lemma: "priděliti", addition: "", transitive: true, imperfective: false },
    ],
    "priděljati" => &[
        VerbDictionaryEntry { lemma: "priděljati", addition: "", transitive: true, imperfective: true },
    ],
    "priględati" => &[
        VerbDictionaryEntry { lemma: "priględati", addition: "", transitive: true, imperfective: true },
    ],
    "priględěti" => &[
        VerbDictionaryEntry { lemma: "priględěti", addition: "(priględi)", transitive: true, imperfective: false },
    ],
    "prignųti" => &[
        VerbDictionaryEntry { lemma: "prignųti", addition: "", transitive: true, imperfective: false },
    ],
    "prigotoviti" => &[
        VerbDictionaryEntry { lemma: "prigotoviti", addition: "", transitive: true, imperfective: false },
    ],
    "prigybati" => &[
        VerbDictionaryEntry { lemma: "prigybati", addition: "", transitive: true, imperfective: true },
    ],
    "prihoditi" => &[
        VerbDictionaryEntry { lemma: "prihoditi", addition: "", transitive: true, imperfective: true },
    ],
    "prijati" => &[
        VerbDictionaryEntry { lemma: "prijati", addition: "(+3)", transitive: true, imperfective: true },
    ],
    "prijdti" => &[
        VerbDictionaryEntry { lemma: "prijdti", addition: "(prijde; prišėl)", transitive: true, imperfective: false },
    ],
    "prijehati" => &[
        VerbDictionaryEntry { lemma: "prijehati", addition: "(prijede)", transitive: true, imperfective: false },
    ],
    "priježđati" => &[
        VerbDictionaryEntry { lemma: "priježđati", addition: "", transitive: true, imperfective: true },
    ],
    "prijmati" => &[
        VerbDictionaryEntry { lemma: "prijmati", addition: "", transitive: true, imperfective: true },
    ],
    "prijęti" => &[
        VerbDictionaryEntry { lemma: "prijęti", addition: "(prijme)", transitive: true, imperfective: false },
    ],
    "prikazati" => &[
        VerbDictionaryEntry { lemma: "prikazati", addition: "(prikaže)", transitive: true, imperfective: false },
    ],
    "prikazyvati" => &[
        VerbDictionaryEntry { lemma: "prikazyvati", addition: "", transitive: true, imperfective: true },
    ],
    "prikladati" => &[
        VerbDictionaryEntry { lemma: "prikladati", addition: "", transitive: true, imperfective: true },
    ],
    "priletěti" => &[
        VerbDictionaryEntry { lemma: "priletěti", addition: "(prilěti)", transitive: true, imperfective: false },
    ],
    "priložiti" => &[
        VerbDictionaryEntry { lemma: "priložiti", addition: "", transitive: true, imperfective: false },
    ],
    "prilěpiti" => &[
        VerbDictionaryEntry { lemma: "prilěpiti", addition: "", transitive: true, imperfective: false },
    ],
    "prilětati" => &[
        VerbDictionaryEntry { lemma: "prilětati", addition: "", transitive: true, imperfective: true },
    ],
    "prilųčati" => &[
        VerbDictionaryEntry { lemma: "prilųčati", addition: "", transitive: true, imperfective: true },
    ],
    "prilųčiti" => &[
        VerbDictionaryEntry { lemma: "prilųčiti", addition: "", transitive: true, imperfective: false },
    ],
    "priměniti" => &[
        VerbDictionaryEntry { lemma: "priměniti", addition: "", transitive: true, imperfective: false },
    ],
    "priměnjati" => &[
        VerbDictionaryEntry { lemma: "priměnjati", addition: "", transitive: true, imperfective: true },
    ],
    "primětiti" => &[
        VerbDictionaryEntry { lemma: "primětiti", addition: "", transitive: true, imperfective: false },
    ],
    "priměćati" => &[
        VerbDictionaryEntry { lemma: "priměćati", addition: "", transitive: true, imperfective: true },
    ],
    "prinaležati" => &[
        VerbDictionaryEntry { lemma: "prinaležati", addition: "(prinaleži)", transitive: true, imperfective: true },
    ],
    "prinesti" => &[
        VerbDictionaryEntry { lemma: "prinesti", addition: "", transitive: true, imperfective: false },
    ],
    "prinositi" => &[
        VerbDictionaryEntry { lemma: "prinositi", addition: "", transitive: true, imperfective: true },
    ],
    "prinuditi" => &[
        VerbDictionaryEntry { lemma: "prinuditi", addition: "", transitive: true, imperfective: false },
    ],
    "prinuđati" => &[
        VerbDictionaryEntry { lemma: "prinuđati", addition: "", transitive: true, imperfective: true },
    ],
    "pripadati" => &[
        VerbDictionaryEntry { lemma: "pripadati", addition: "", transitive: true, imperfective: true },
    ],
    "pripasti" => &[
        VerbDictionaryEntry { lemma: "pripasti", addition: "(pripade)", transitive: true, imperfective: false },
    ],
    "pripinati" => &[
        VerbDictionaryEntry { lemma: "pripinati", addition: "", transitive: true, imperfective: true },
    ],
    "pripisati" => &[
        VerbDictionaryEntry { lemma: "pripisati", addition: "(pripiše)", transitive: true, imperfective: false },
    ],
    "pripisyvati" => &[
        VerbDictionaryEntry { lemma: "pripisyvati", addition: "", transitive: true, imperfective: true },
    ],
    "pripluti" => &[
        VerbDictionaryEntry { lemma: "pripluti", addition: "(priplove)", transitive: true, imperfective: false },
    ],
    "priplyvati" => &[
        VerbDictionaryEntry { lemma: "priplyvati", addition: "", transitive: true, imperfective: true },
    ],
    "priplyvti" => &[
        VerbDictionaryEntry { lemma: "priplyvti", addition: "", transitive: true, imperfective: false },
    ],
    "pripominati" => &[
        VerbDictionaryEntry { lemma: "pripominati", addition: "", transitive: true, imperfective: true },
    ],
    "pripomněti" => &[
        VerbDictionaryEntry { lemma: "pripomněti", addition: "(pripomni)", transitive: true, imperfective: false },
    ],
    "pripraviti" => &[
        VerbDictionaryEntry { lemma: "pripraviti", addition: "", transitive: true, imperfective: false },
    ],
    "pripravjati" => &[
        VerbDictionaryEntry { lemma: "pripravjati", addition: "", transitive: true, imperfective: true },
    ],
    "pripustiti" => &[
        VerbDictionaryEntry { lemma: "pripustiti", addition: "", transitive: true, imperfective: false },
    ],
    "pripušćati" => &[
        VerbDictionaryEntry { lemma: "pripušćati", addition: "", transitive: true, imperfective: true },
    ],
    "pripęti" => &[
        VerbDictionaryEntry { lemma: "pripęti", addition: "(pripne)", transitive: true, imperfective: false },
    ],
    "prisjediniti" => &[
        VerbDictionaryEntry { lemma: "prisjediniti", addition: "", transitive: true, imperfective: false },
    ],
    "prisjedinjati" => &[
        VerbDictionaryEntry { lemma: "prisjedinjati", addition: "", transitive: true, imperfective: true },
    ],
    "prislati" => &[
        VerbDictionaryEntry { lemma: "prislati", addition: "(prišlje)", transitive: true, imperfective: false },
    ],
    "prisposobiti" => &[
        VerbDictionaryEntry { lemma: "prisposobiti", addition: "", transitive: true, imperfective: false },
    ],
    "prisposobjati" => &[
        VerbDictionaryEntry { lemma: "prisposobjati", addition: "", transitive: true, imperfective: true },
    ],
    "prispěti" => &[
        VerbDictionaryEntry { lemma: "prispěti", addition: "", transitive: true, imperfective: false },
    ],
    "prispěvati" => &[
        VerbDictionaryEntry { lemma: "prispěvati", addition: "", transitive: true, imperfective: true },
    ],
    "pristojati" => &[
        VerbDictionaryEntry { lemma: "pristojati", addition: "(pristoji)", transitive: true, imperfective: true },
    ],
    "pristrigati" => &[
        VerbDictionaryEntry { lemma: "pristrigati", addition: "", transitive: true, imperfective: true },
    ],
    "pristrigti" => &[
        VerbDictionaryEntry { lemma: "pristrigti", addition: "", transitive: true, imperfective: false },
    ],
    "prisvajati" => &[
        VerbDictionaryEntry { lemma: "prisvajati", addition: "", transitive: true, imperfective: true },
    ],
    "prisvojiti" => &[
        VerbDictionaryEntry { lemma: "prisvojiti", addition: "", transitive: true, imperfective: false },
    ],
    "prisylati" => &[
        VerbDictionaryEntry { lemma: "prisylati", addition: "", transitive: true, imperfective: true },
    ],
    "prisęgati" => &[
        VerbDictionaryEntry { lemma: "prisęgati", addition: "", transitive: true, imperfective: true },
    ],
    "prisęgnųti" => &[
        VerbDictionaryEntry { lemma: "prisęgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "prisųditi" => &[
        VerbDictionaryEntry { lemma: "prisųditi", addition: "", transitive: true, imperfective: false },
    ],
    "prisųđati" => &[
        VerbDictionaryEntry { lemma: "prisųđati", addition: "", transitive: true, imperfective: true },
    ],
    "pritiskati" => &[
        VerbDictionaryEntry { lemma: "pritiskati", addition: "", transitive: true, imperfective: true },
    ],
    "pritisknųti" => &[
        VerbDictionaryEntry { lemma: "pritisknųti", addition: "", transitive: true, imperfective: false },
    ],
    "pritęgati" => &[
        VerbDictionaryEntry { lemma: "pritęgati", addition: "", transitive: true, imperfective: true },
    ],
    "pritęgnųti" => &[
        VerbDictionaryEntry { lemma: "pritęgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "priučati" => &[
        VerbDictionaryEntry { lemma: "priučati", addition: "", transitive: true, imperfective: true },
    ],
    "priučiti" => &[
        VerbDictionaryEntry { lemma: "priučiti", addition: "", transitive: true, imperfective: false },
    ],
    "privabiti" => &[
        VerbDictionaryEntry { lemma: "privabiti", addition: "", transitive: true, imperfective: false },
    ],
    "privabjati" => &[
        VerbDictionaryEntry { lemma: "privabjati", addition: "", transitive: true, imperfective: true },
    ],
    "privesti" => &[
        VerbDictionaryEntry { lemma: "privesti", addition: "(privede)", transitive: true, imperfective: false },
    ],
    "privezti" => &[
        VerbDictionaryEntry { lemma: "privezti", addition: "", transitive: true, imperfective: false },
    ],
    "privitati" => &[
        VerbDictionaryEntry { lemma: "privitati", addition: "", transitive: true, imperfective: false },
    ],
    "privlåčivati" => &[
        VerbDictionaryEntry { lemma: "privlåčivati", addition: "", transitive: true, imperfective: true },
    ],
    "privlěkati" => &[
        VerbDictionaryEntry { lemma: "privlěkati", addition: "", transitive: true, imperfective: true },
    ],
    "privlěkti" => &[
        VerbDictionaryEntry { lemma: "privlěkti", addition: "", transitive: true, imperfective: false },
    ],
    "privoditi" => &[
        VerbDictionaryEntry { lemma: "privoditi", addition: "", transitive: true, imperfective: true },
    ],
    "privoliti" => &[
        VerbDictionaryEntry { lemma: "privoliti", addition: "", transitive: false, imperfective: false },
    ],
    "privoljati" => &[
        VerbDictionaryEntry { lemma: "privoljati", addition: "", transitive: false, imperfective: true },
    ],
    "privoziti" => &[
        VerbDictionaryEntry { lemma: "privoziti", addition: "", transitive: true, imperfective: true },
    ],
    "privykati" => &[
        VerbDictionaryEntry { lemma: "privykati", addition: "", transitive: true, imperfective: true },
    ],
    "privyknųti" => &[
        VerbDictionaryEntry { lemma: "privyknųti", addition: "", transitive: true, imperfective: false },
    ],
    "privęzati" => &[
        VerbDictionaryEntry { lemma: "privęzati", addition: "(privęže)", transitive: true, imperfective: false },
    ],
    "privęzyvati" => &[
        VerbDictionaryEntry { lemma: "privęzyvati", addition: "", transitive: true, imperfective: true },
    ],
    "priznati" => &[
        VerbDictionaryEntry { lemma: "priznati", addition: "", transitive: true, imperfective: false },
    ],
    "priznavati" => &[
        VerbDictionaryEntry { lemma: "priznavati", addition: "", transitive: true, imperfective: true },
    ],
    "prizvati" => &[
        VerbDictionaryEntry { lemma: "prizvati", addition: "(prizȯve)", transitive: true, imperfective: false },
    ],
    "prizyvati" => &[
        VerbDictionaryEntry { lemma: "prizyvati", addition: "", transitive: true, imperfective: true },
    ],
    "pričiniti" => &[
        VerbDictionaryEntry { lemma: "pričiniti", addition: "", transitive: false, imperfective: false },
    ],
    "pričinjati" => &[
        VerbDictionaryEntry { lemma: "pričinjati", addition: "", transitive: false, imperfective: true },
    ],
    "prišiti" => &[
        VerbDictionaryEntry { lemma: "prišiti", addition: "(prišije)", transitive: true, imperfective: false },
    ],
    "probiti" => &[
        VerbDictionaryEntry { lemma: "probiti", addition: "(probije)", transitive: true, imperfective: false },
    ],
    "probivati" => &[
        VerbDictionaryEntry { lemma: "probivati", addition: "", transitive: true, imperfective: true },
    ],
    "probovati" => &[
        VerbDictionaryEntry { lemma: "probovati", addition: "", transitive: true, imperfective: true },
    ],
    "procitovati" => &[
        VerbDictionaryEntry { lemma: "procitovati", addition: "", transitive: true, imperfective: false },
    ],
    "procvětati" => &[
        VerbDictionaryEntry { lemma: "procvětati", addition: "", transitive: true, imperfective: true },
    ],
    "prodati" => &[
        VerbDictionaryEntry { lemma: "prodati", addition: "(proda)", transitive: true, imperfective: false },
    ],
    "prodavati" => &[
        VerbDictionaryEntry { lemma: "prodavati", addition: "", transitive: true, imperfective: true },
    ],
    "prodiraviti" => &[
        VerbDictionaryEntry { lemma: "prodiraviti", addition: "", transitive: true, imperfective: false },
    ],
    "prodiravjati" => &[
        VerbDictionaryEntry { lemma: "prodiravjati", addition: "", transitive: true, imperfective: true },
    ],
    "produkovati" => &[
        VerbDictionaryEntry { lemma: "produkovati", addition: "", transitive: true, imperfective: true },
    ],
    "produmati" => &[
        VerbDictionaryEntry { lemma: "produmati", addition: "", transitive: true, imperfective: false },
    ],
    "prodȯlžati" => &[
        VerbDictionaryEntry { lemma: "prodȯlžati", addition: "", transitive: true, imperfective: true },
    ],
    "prodȯlžiti" => &[
        VerbDictionaryEntry { lemma: "prodȯlžiti", addition: "", transitive: true, imperfective: false },
    ],
    "profanovati" => &[
        VerbDictionaryEntry { lemma: "profanovati", addition: "", transitive: true, imperfective: true },
    ],
    "profesionalizovati" => &[
        VerbDictionaryEntry { lemma: "profesionalizovati", addition: "", transitive: true, imperfective: true },
    ],
    "proganjati" => &[
        VerbDictionaryEntry { lemma: "proganjati", addition: "", transitive: true, imperfective: true },
    ],
    "proglašati" => &[
        VerbDictionaryEntry { lemma: "proglašati", addition: "", transitive: true, imperfective: true },
    ],
    "proglåsiti" => &[
        VerbDictionaryEntry { lemma: "proglåsiti", addition: "", transitive: true, imperfective: false },
    ],
    "programovati" => &[
        VerbDictionaryEntry { lemma: "programovati", addition: "", transitive: true, imperfective: true },
    ],
    "progȯltati" => &[
        VerbDictionaryEntry { lemma: "progȯltati", addition: "", transitive: true, imperfective: true },
    ],
    "progȯltnųti" => &[
        VerbDictionaryEntry { lemma: "progȯltnųti", addition: "", transitive: true, imperfective: false },
    ],
    "prohlåditi" => &[
        VerbDictionaryEntry { lemma: "prohlåditi", addition: "", transitive: true, imperfective: false },
    ],
    "prohoditi" => &[
        VerbDictionaryEntry { lemma: "prohoditi", addition: "", transitive: true, imperfective: true },
    ],
    "proigrati" => &[
        VerbDictionaryEntry { lemma: "proigrati", addition: "", transitive: true, imperfective: false },
    ],
    "proigryvati" => &[
        VerbDictionaryEntry { lemma: "proigryvati", addition: "", transitive: true, imperfective: true },
    ],
    "proiztekti" => &[
        VerbDictionaryEntry { lemma: "proiztekti", addition: "", transitive: true, imperfective: false },
    ],
    "proiztěkati" => &[
        VerbDictionaryEntry { lemma: "proiztěkati", addition: "", transitive: true, imperfective: true },
    ],
    "proizvesti" => &[
        VerbDictionaryEntry { lemma: "proizvesti", addition: "(proizvede)", transitive: true, imperfective: false },
    ],
    "proizvoditi" => &[
        VerbDictionaryEntry { lemma: "proizvoditi", addition: "", transitive: true, imperfective: true },
    ],
    "projaviti" => &[
        VerbDictionaryEntry { lemma: "projaviti", addition: "", transitive: true, imperfective: false },
    ],
    "projavjati" => &[
        VerbDictionaryEntry { lemma: "projavjati", addition: "", transitive: true, imperfective: true },
    ],
    "projdti" => &[
        VerbDictionaryEntry { lemma: "projdti", addition: "(projde; prošėl)", transitive: true, imperfective: false },
    ],
    "projehati" => &[
        VerbDictionaryEntry { lemma: "projehati", addition: "(projede)", transitive: true, imperfective: false },
    ],
    "projektovati" => &[
        VerbDictionaryEntry { lemma: "projektovati", addition: "", transitive: true, imperfective: true },
    ],
    "proježđati" => &[
        VerbDictionaryEntry { lemma: "proježđati", addition: "", transitive: true, imperfective: true },
    ],
    "projmati" => &[
        VerbDictionaryEntry { lemma: "projmati", addition: "", transitive: true, imperfective: true },
    ],
    "projęti" => &[
        VerbDictionaryEntry { lemma: "projęti", addition: "(projme)", transitive: true, imperfective: false },
    ],
    "proklęti" => &[
        VerbDictionaryEntry { lemma: "proklęti", addition: "(proklne)", transitive: true, imperfective: false },
    ],
    "prokontrolovati" => &[
        VerbDictionaryEntry { lemma: "prokontrolovati", addition: "", transitive: true, imperfective: false },
    ],
    "prokrijumčariti" => &[
        VerbDictionaryEntry { lemma: "prokrijumčariti", addition: "", transitive: true, imperfective: false },
    ],
    "prolamyvati" => &[
        VerbDictionaryEntry { lemma: "prolamyvati", addition: "", transitive: true, imperfective: true },
    ],
    "proliti" => &[
        VerbDictionaryEntry { lemma: "proliti", addition: "(prolije)", transitive: true, imperfective: false },
    ],
    "prolivati" => &[
        VerbDictionaryEntry { lemma: "prolivati", addition: "", transitive: true, imperfective: true },
    ],
    "prolomiti" => &[
        VerbDictionaryEntry { lemma: "prolomiti", addition: "", transitive: true, imperfective: false },
    ],
    "prolězati" => &[
        VerbDictionaryEntry { lemma: "prolězati", addition: "", transitive: true, imperfective: true },
    ],
    "prolězti" => &[
        VerbDictionaryEntry { lemma: "prolězti", addition: "", transitive: true, imperfective: false },
    ],
    "pronevěriti" => &[
        VerbDictionaryEntry { lemma: "pronevěriti", addition: "", transitive: true, imperfective: false },
    ],
    "pronevěrjati" => &[
        VerbDictionaryEntry { lemma: "pronevěrjati", addition: "", transitive: true, imperfective: true },
    ],
    "pronikati" => &[
        VerbDictionaryEntry { lemma: "pronikati", addition: "", transitive: true, imperfective: true },
    ],
    "proniknųti" => &[
        VerbDictionaryEntry { lemma: "proniknųti", addition: "", transitive: true, imperfective: false },
    ],
    "propadati" => &[
        VerbDictionaryEntry { lemma: "propadati", addition: "", transitive: true, imperfective: true },
    ],
    "propagovati" => &[
        VerbDictionaryEntry { lemma: "propagovati", addition: "", transitive: true, imperfective: true },
    ],
    "propasti" => &[
        VerbDictionaryEntry { lemma: "propasti", addition: "(propade)", transitive: true, imperfective: false },
    ],
    "propiti" => &[
        VerbDictionaryEntry { lemma: "propiti", addition: "", transitive: true, imperfective: false },
    ],
    "propivati" => &[
        VerbDictionaryEntry { lemma: "propivati", addition: "", transitive: true, imperfective: true },
    ],
    "proponovati" => &[
        VerbDictionaryEntry { lemma: "proponovati", addition: "", transitive: true, imperfective: true },
    ],
    "propustiti" => &[
        VerbDictionaryEntry { lemma: "propustiti", addition: "", transitive: true, imperfective: false },
    ],
    "propušćati" => &[
        VerbDictionaryEntry { lemma: "propušćati", addition: "", transitive: true, imperfective: true },
    ],
    "proråstati" => &[
        VerbDictionaryEntry { lemma: "proråstati", addition: "", transitive: true, imperfective: true },
    ],
    "prositi" => &[
        VerbDictionaryEntry { lemma: "prositi", addition: "", transitive: true, imperfective: true },
    ],
    "proslaviti" => &[
        VerbDictionaryEntry { lemma: "proslaviti", addition: "", transitive: true, imperfective: false },
    ],
    "proslavjati" => &[
        VerbDictionaryEntry { lemma: "proslavjati", addition: "", transitive: true, imperfective: true },
    ],
    "prospati" => &[
        VerbDictionaryEntry { lemma: "prospati", addition: "(prospi)", transitive: true, imperfective: false },
    ],
    "prostirati" => &[
        VerbDictionaryEntry { lemma: "prostirati", addition: "", transitive: true, imperfective: true },
    ],
    "prostiti" => &[
        VerbDictionaryEntry { lemma: "prostiti", addition: "", transitive: true, imperfective: false },
    ],
    "prostrěti" => &[
        VerbDictionaryEntry { lemma: "prostrěti", addition: "(prostre)", transitive: true, imperfective: false },
    ],
    "prostŕti" => &[
        VerbDictionaryEntry { lemma: "prostŕti", addition: "(prostre)", transitive: true, imperfective: false },
    ],
    "prosvětiti" => &[
        VerbDictionaryEntry { lemma: "prosvětiti", addition: "", transitive: true, imperfective: false },
    ],
    "prosvěćati" => &[
        VerbDictionaryEntry { lemma: "prosvěćati", addition: "", transitive: true, imperfective: true },
    ],
    "protekti" => &[
        VerbDictionaryEntry { lemma: "protekti", addition: "", transitive: true, imperfective: false },
    ],
    "protestovati" => &[
        VerbDictionaryEntry { lemma: "protestovati", addition: "", transitive: true, imperfective: true },
    ],
    "protivdějati" => &[
        VerbDictionaryEntry { lemma: "protivdějati", addition: "", transitive: true, imperfective: true },
    ],
    "protivrěčiti" => &[
        VerbDictionaryEntry { lemma: "protivrěčiti", addition: "", transitive: true, imperfective: true },
    ],
    "protrěti" => &[
        VerbDictionaryEntry { lemma: "protrěti", addition: "(protre)", transitive: true, imperfective: false },
    ],
    "protěkati" => &[
        VerbDictionaryEntry { lemma: "protěkati", addition: "", transitive: true, imperfective: true },
    ],
    "protŕti" => &[
        VerbDictionaryEntry { lemma: "protŕti", addition: "(protre)", transitive: true, imperfective: false },
    ],
    "provesti" => &[
        VerbDictionaryEntry { lemma: "provesti", addition: "(provede)", transitive: true, imperfective: false },
    ],
    "provoditi" => &[
        VerbDictionaryEntry { lemma: "provoditi", addition: "", transitive: true, imperfective: true },
    ],
    "provokovati" => &[
        VerbDictionaryEntry { lemma: "provokovati", addition: "", transitive: true, imperfective: true },
    ],
    "provođati" => &[
        VerbDictionaryEntry { lemma: "provođati", addition: "", transitive: true, imperfective: true },
    ],
    "prověriti" => &[
        VerbDictionaryEntry { lemma: "prověriti", addition: "", transitive: true, imperfective: false },
    ],
    "prověrjati" => &[
        VerbDictionaryEntry { lemma: "prověrjati", addition: "", transitive: true, imperfective: true },
    ],
    "provětriti" => &[
        VerbDictionaryEntry { lemma: "provětriti", addition: "", transitive: true, imperfective: false },
    ],
    "provětrjati" => &[
        VerbDictionaryEntry { lemma: "provětrjati", addition: "", transitive: true, imperfective: true },
    ],
    "provŕgati" => &[
        VerbDictionaryEntry { lemma: "provŕgati", addition: "", transitive: true, imperfective: true },
    ],
    "provŕgnųti" => &[
        VerbDictionaryEntry { lemma: "provŕgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "provŕtiti" => &[
        VerbDictionaryEntry { lemma: "provŕtiti", addition: "", transitive: true, imperfective: false },
    ],
    "provŕćati" => &[
        VerbDictionaryEntry { lemma: "provŕćati", addition: "", transitive: true, imperfective: true },
    ],
    "pročistiti" => &[
        VerbDictionaryEntry { lemma: "pročistiti", addition: "", transitive: true, imperfective: false },
    ],
    "pročitati" => &[
        VerbDictionaryEntry { lemma: "pročitati", addition: "", transitive: true, imperfective: false },
    ],
    "pročišćati" => &[
        VerbDictionaryEntry { lemma: "pročišćati", addition: "", transitive: true, imperfective: true },
    ],
    "prošćati" => &[
        VerbDictionaryEntry { lemma: "prošćati", addition: "", transitive: true, imperfective: true },
    ],
    "proživati" => &[
        VerbDictionaryEntry { lemma: "proživati", addition: "", transitive: true, imperfective: true },
    ],
    "prskati" => &[
        VerbDictionaryEntry { lemma: "prskati", addition: "", transitive: true, imperfective: true },
    ],
    "prsknųti" => &[
        VerbDictionaryEntry { lemma: "prsknųti", addition: "", transitive: true, imperfective: false },
    ],
    "pråti" => &[
        VerbDictionaryEntry { lemma: "pråti", addition: "(poŕe)", transitive: true, imperfective: true },
    ],
    "pråzdnovati" => &[
        VerbDictionaryEntry { lemma: "pråzdnovati", addition: "", transitive: true, imperfective: true },
    ],
    "pråšiti" => &[
        VerbDictionaryEntry { lemma: "pråšiti", addition: "", transitive: false, imperfective: true },
    ],
    "pręsti" => &[
        VerbDictionaryEntry { lemma: "pręsti", addition: "(pręde)", transitive: true, imperfective: true },
    ],
    "prěadresovati" => &[
        VerbDictionaryEntry { lemma: "prěadresovati", addition: "", transitive: true, imperfective: false },
    ],
    "prěbudovati" => &[
        VerbDictionaryEntry { lemma: "prěbudovati", addition: "", transitive: true, imperfective: false },
    ],
    "prěbudovyvati" => &[
        VerbDictionaryEntry { lemma: "prěbudovyvati", addition: "", transitive: true, imperfective: true },
    ],
    "prěbyvati" => &[
        VerbDictionaryEntry { lemma: "prěbyvati", addition: "", transitive: true, imperfective: true },
    ],
    "prěcěniti" => &[
        VerbDictionaryEntry { lemma: "prěcěniti", addition: "", transitive: true, imperfective: false },
    ],
    "prěcěnjati" => &[
        VerbDictionaryEntry { lemma: "prěcěnjati", addition: "", transitive: true, imperfective: true },
    ],
    "prědati" => &[
        VerbDictionaryEntry { lemma: "prědati", addition: "(prěda)", transitive: true, imperfective: false },
    ],
    "prědavati" => &[
        VerbDictionaryEntry { lemma: "prědavati", addition: "", transitive: true, imperfective: true },
    ],
    "prědhoditi" => &[
        VerbDictionaryEntry { lemma: "prědhoditi", addition: "", transitive: true, imperfective: true },
    ],
    "prědkladati" => &[
        VerbDictionaryEntry { lemma: "prědkladati", addition: "", transitive: true, imperfective: true },
    ],
    "prědlagati" => &[
        VerbDictionaryEntry { lemma: "prědlagati", addition: "", transitive: true, imperfective: true },
    ],
    "prědložiti" => &[
        VerbDictionaryEntry { lemma: "prědložiti", addition: "", transitive: true, imperfective: false },
    ],
    "prědpisati" => &[
        VerbDictionaryEntry { lemma: "prědpisati", addition: "(prědpiše)", transitive: true, imperfective: false },
    ],
    "prědpisyvati" => &[
        VerbDictionaryEntry { lemma: "prědpisyvati", addition: "", transitive: true, imperfective: true },
    ],
    "prědplatiti" => &[
        VerbDictionaryEntry { lemma: "prědplatiti", addition: "", transitive: true, imperfective: false },
    ],
    "prědplaćati" => &[
        VerbDictionaryEntry { lemma: "prědplaćati", addition: "", transitive: true, imperfective: true },
    ],
    "prědpokladati" => &[
        VerbDictionaryEntry { lemma: "prědpokladati", addition: "", transitive: true, imperfective: true },
    ],
    "prědpolagati" => &[
        VerbDictionaryEntry { lemma: "prědpolagati", addition: "", transitive: true, imperfective: true },
    ],
    "prědpoložiti" => &[
        VerbDictionaryEntry { lemma: "prědpoložiti", addition: "", transitive: true, imperfective: false },
    ],
    "prědpovědati" => &[
        VerbDictionaryEntry { lemma: "prědpovědati", addition: "", transitive: true, imperfective: true },
    ],
    "prědpověděti" => &[
        VerbDictionaryEntry { lemma: "prědpověděti", addition: "(prědpově)", transitive: true, imperfective: false },
    ],
    "prědpočitati" => &[
        VerbDictionaryEntry { lemma: "prědpočitati", addition: "", transitive: true, imperfective: true },
    ],
    "prědprijmati" => &[
        VerbDictionaryEntry { lemma: "prědprijmati", addition: "", transitive: true, imperfective: true },
    ],
    "prědprijęti" => &[
        VerbDictionaryEntry { lemma: "prědprijęti", addition: "(prědprijme)", transitive: true, imperfective: false },
    ],
    "prědskazati" => &[
        VerbDictionaryEntry { lemma: "prědskazati", addition: "(prědskaže)", transitive: true, imperfective: false },
    ],
    "prědskazyvati" => &[
        VerbDictionaryEntry { lemma: "prědskazyvati", addition: "", transitive: true, imperfective: true },
    ],
    "prědstati" => &[
        VerbDictionaryEntry { lemma: "prědstati", addition: "(prědstane)", transitive: true, imperfective: false },
    ],
    "prědstavati" => &[
        VerbDictionaryEntry { lemma: "prědstavati", addition: "", transitive: true, imperfective: true },
    ],
    "prědstaviti" => &[
        VerbDictionaryEntry { lemma: "prědstaviti", addition: "", transitive: true, imperfective: false },
    ],
    "prědstavjati" => &[
        VerbDictionaryEntry { lemma: "prědstavjati", addition: "", transitive: true, imperfective: true },
    ],
    "prědsědati" => &[
        VerbDictionaryEntry { lemma: "prědsědati", addition: "", transitive: true, imperfective: true },
    ],
    "prěduprěditi" => &[
        VerbDictionaryEntry { lemma: "prěduprěditi", addition: "", transitive: true, imperfective: false },
    ],
    "prěduprěđati" => &[
        VerbDictionaryEntry { lemma: "prěduprěđati", addition: "", transitive: true, imperfective: true },
    ],
    "prědvidyvati" => &[
        VerbDictionaryEntry { lemma: "prědvidyvati", addition: "", transitive: true, imperfective: true },
    ],
    "prědviděti" => &[
        VerbDictionaryEntry { lemma: "prědviděti", addition: "(prědvidi)", transitive: true, imperfective: false },
    ],
    "prědvyšati" => &[
        VerbDictionaryEntry { lemma: "prědvyšati", addition: "", transitive: true, imperfective: true },
    ],
    "prědčuvati" => &[
        VerbDictionaryEntry { lemma: "prědčuvati", addition: "", transitive: true, imperfective: true },
    ],
    "prědȯjdti" => &[
        VerbDictionaryEntry { lemma: "prědȯjdti", addition: "(prědȯjde; prědšėl)", transitive: true, imperfective: false },
    ],
    "prěględati" => &[
        VerbDictionaryEntry { lemma: "prěględati", addition: "", transitive: true, imperfective: true },
    ],
    "prěględěti" => &[
        VerbDictionaryEntry { lemma: "prěględěti", addition: "(prěględi)", transitive: true, imperfective: false },
    ],
    "prěhoditi" => &[
        VerbDictionaryEntry { lemma: "prěhoditi", addition: "", transitive: true, imperfective: true },
    ],
    "prěhytriti" => &[
        VerbDictionaryEntry { lemma: "prěhytriti", addition: "", transitive: true, imperfective: false },
    ],
    "prěhytrjati" => &[
        VerbDictionaryEntry { lemma: "prěhytrjati", addition: "", transitive: true, imperfective: true },
    ],
    "prěimenovati" => &[
        VerbDictionaryEntry { lemma: "prěimenovati", addition: "", transitive: true, imperfective: false },
    ],
    "prěimenovyvati" => &[
        VerbDictionaryEntry { lemma: "prěimenovyvati", addition: "", transitive: true, imperfective: true },
    ],
    "prěinačati" => &[
        VerbDictionaryEntry { lemma: "prěinačati", addition: "", transitive: true, imperfective: true },
    ],
    "prěinačiti" => &[
        VerbDictionaryEntry { lemma: "prěinačiti", addition: "", transitive: true, imperfective: false },
    ],
    "prějdti" => &[
        VerbDictionaryEntry { lemma: "prějdti", addition: "(prějde; prěšėl)", transitive: true, imperfective: false },
    ],
    "prějmati" => &[
        VerbDictionaryEntry { lemma: "prějmati", addition: "", transitive: true, imperfective: true },
    ],
    "prějęti" => &[
        VerbDictionaryEntry { lemma: "prějęti", addition: "(prějme)", transitive: true, imperfective: false },
    ],
    "prěkladati" => &[
        VerbDictionaryEntry { lemma: "prěkladati", addition: "", transitive: true, imperfective: true },
    ],
    "prěključati" => &[
        VerbDictionaryEntry { lemma: "prěključati", addition: "", transitive: true, imperfective: true },
    ],
    "prěključiti" => &[
        VerbDictionaryEntry { lemma: "prěključiti", addition: "", transitive: true, imperfective: false },
    ],
    "prěložiti" => &[
        VerbDictionaryEntry { lemma: "prěložiti", addition: "", transitive: true, imperfective: false },
    ],
    "prělězati" => &[
        VerbDictionaryEntry { lemma: "prělězati", addition: "", transitive: true, imperfective: true },
    ],
    "prělězti" => &[
        VerbDictionaryEntry { lemma: "prělězti", addition: "", transitive: true, imperfective: false },
    ],
    "prěmagati" => &[
        VerbDictionaryEntry { lemma: "prěmagati", addition: "", transitive: true, imperfective: true },
    ],
    "prěmeblovati" => &[
        VerbDictionaryEntry { lemma: "prěmeblovati", addition: "", transitive: true, imperfective: true },
    ],
    "prěmogti" => &[
        VerbDictionaryEntry { lemma: "prěmogti", addition: "", transitive: true, imperfective: false },
    ],
    "prěmotati" => &[
        VerbDictionaryEntry { lemma: "prěmotati", addition: "", transitive: true, imperfective: false },
    ],
    "prěmotyvati" => &[
        VerbDictionaryEntry { lemma: "prěmotyvati", addition: "", transitive: true, imperfective: true },
    ],
    "prěměniti" => &[
        VerbDictionaryEntry { lemma: "prěměniti", addition: "", transitive: true, imperfective: false },
    ],
    "prěměnjati" => &[
        VerbDictionaryEntry { lemma: "prěměnjati", addition: "", transitive: true, imperfective: true },
    ],
    "prěměstiti" => &[
        VerbDictionaryEntry { lemma: "prěměstiti", addition: "", transitive: true, imperfective: false },
    ],
    "prěměšćati" => &[
        VerbDictionaryEntry { lemma: "prěměšćati", addition: "", transitive: true, imperfective: true },
    ],
    "prěmȯlviti" => &[
        VerbDictionaryEntry { lemma: "prěmȯlviti", addition: "", transitive: true, imperfective: false },
    ],
    "prěmȯlvjati" => &[
        VerbDictionaryEntry { lemma: "prěmȯlvjati", addition: "", transitive: true, imperfective: true },
    ],
    "prěnapinati" => &[
        VerbDictionaryEntry { lemma: "prěnapinati", addition: "", transitive: true, imperfective: true },
    ],
    "prěnapęti" => &[
        VerbDictionaryEntry { lemma: "prěnapęti", addition: "", transitive: true, imperfective: false },
    ],
    "prěnebrěgati" => &[
        VerbDictionaryEntry { lemma: "prěnebrěgati", addition: "", transitive: true, imperfective: true },
    ],
    "prěnebrěgti" => &[
        VerbDictionaryEntry { lemma: "prěnebrěgti", addition: "(prěnebrěže)", transitive: true, imperfective: false },
    ],
    "prěnesti" => &[
        VerbDictionaryEntry { lemma: "prěnesti", addition: "", transitive: true, imperfective: false },
    ],
    "prěnositi" => &[
        VerbDictionaryEntry { lemma: "prěnositi", addition: "", transitive: true, imperfective: true },
    ],
    "prěnoćevati" => &[
        VerbDictionaryEntry { lemma: "prěnoćevati", addition: "", transitive: true, imperfective: false },
    ],
    "prěobraziti" => &[
        VerbDictionaryEntry { lemma: "prěobraziti", addition: "", transitive: true, imperfective: false },
    ],
    "prěobraćati" => &[
        VerbDictionaryEntry { lemma: "prěobraćati", addition: "", transitive: true, imperfective: true },
    ],
    "prěobražati" => &[
        VerbDictionaryEntry { lemma: "prěobražati", addition: "", transitive: true, imperfective: true },
    ],
    "prěobråtiti" => &[
        VerbDictionaryEntry { lemma: "prěobråtiti", addition: "", transitive: true, imperfective: false },
    ],
    "prěocěniti" => &[
        VerbDictionaryEntry { lemma: "prěocěniti", addition: "", transitive: true, imperfective: false },
    ],
    "prěocěnjati" => &[
        VerbDictionaryEntry { lemma: "prěocěnjati", addition: "", transitive: true, imperfective: true },
    ],
    "prěpisati" => &[
        VerbDictionaryEntry { lemma: "prěpisati", addition: "(prěpiše)", transitive: true, imperfective: false },
    ],
    "prěpisyvati" => &[
        VerbDictionaryEntry { lemma: "prěpisyvati", addition: "", transitive: true, imperfective: true },
    ],
    "prěpluti" => &[
        VerbDictionaryEntry { lemma: "prěpluti", addition: "(prěplove)", transitive: true, imperfective: false },
    ],
    "prěplyvati" => &[
        VerbDictionaryEntry { lemma: "prěplyvati", addition: "", transitive: true, imperfective: true },
    ],
    "prěplyvti" => &[
        VerbDictionaryEntry { lemma: "prěplyvti", addition: "", transitive: true, imperfective: false },
    ],
    "prěporųčati" => &[
        VerbDictionaryEntry { lemma: "prěporųčati", addition: "", transitive: true, imperfective: true },
    ],
    "prěporųčiti" => &[
        VerbDictionaryEntry { lemma: "prěporųčiti", addition: "", transitive: true, imperfective: false },
    ],
    "prěpraviti" => &[
        VerbDictionaryEntry { lemma: "prěpraviti", addition: "", transitive: true, imperfective: false },
    ],
    "prěpravjati" => &[
        VerbDictionaryEntry { lemma: "prěpravjati", addition: "", transitive: true, imperfective: true },
    ],
    "prěprogramovati" => &[
        VerbDictionaryEntry { lemma: "prěprogramovati", addition: "", transitive: true, imperfective: false },
    ],
    "prěrastati" => &[
        VerbDictionaryEntry { lemma: "prěrastati", addition: "", transitive: true, imperfective: true },
    ],
    "prěrvati" => &[
        VerbDictionaryEntry { lemma: "prěrvati", addition: "(prěrve)", transitive: true, imperfective: false },
    ],
    "prěryvati" => &[
        VerbDictionaryEntry { lemma: "prěryvati", addition: "", transitive: true, imperfective: true },
    ],
    "prěråsti" => &[
        VerbDictionaryEntry { lemma: "prěråsti", addition: "(prěråste)", transitive: true, imperfective: false },
    ],
    "prěråzkazati" => &[
        VerbDictionaryEntry { lemma: "prěråzkazati", addition: "(prěskaže)", transitive: true, imperfective: false },
    ],
    "prěråzkazyvati" => &[
        VerbDictionaryEntry { lemma: "prěråzkazyvati", addition: "", transitive: true, imperfective: true },
    ],
    "prěrězati" => &[
        VerbDictionaryEntry { lemma: "prěrězati", addition: "(prěrěže)", transitive: true, imperfective: false },
    ],
    "prěseliti" => &[
        VerbDictionaryEntry { lemma: "prěseliti", addition: "", transitive: true, imperfective: false },
    ],
    "prěseljati" => &[
        VerbDictionaryEntry { lemma: "prěseljati", addition: "", transitive: true, imperfective: true },
    ],
    "prěslušati" => &[
        VerbDictionaryEntry { lemma: "prěslušati", addition: "", transitive: true, imperfective: false },
    ],
    "prěslušivati" => &[
        VerbDictionaryEntry { lemma: "prěslušivati", addition: "", transitive: true, imperfective: true },
    ],
    "prěslědovati" => &[
        VerbDictionaryEntry { lemma: "prěslědovati", addition: "", transitive: true, imperfective: true },
    ],
    "prěslěpiti" => &[
        VerbDictionaryEntry { lemma: "prěslěpiti", addition: "", transitive: true, imperfective: false },
    ],
    "prěsměriti" => &[
        VerbDictionaryEntry { lemma: "prěsměriti", addition: "", transitive: true, imperfective: false },
    ],
    "prěsměrjati" => &[
        VerbDictionaryEntry { lemma: "prěsměrjati", addition: "", transitive: true, imperfective: true },
    ],
    "prěspati" => &[
        VerbDictionaryEntry { lemma: "prěspati", addition: "(prěspi)", transitive: true, imperfective: false },
    ],
    "prěstati" => &[
        VerbDictionaryEntry { lemma: "prěstati", addition: "(prěstane)", transitive: true, imperfective: false },
    ],
    "prěstavati" => &[
        VerbDictionaryEntry { lemma: "prěstavati", addition: "", transitive: true, imperfective: true },
    ],
    "prěstaviti" => &[
        VerbDictionaryEntry { lemma: "prěstaviti", addition: "", transitive: true, imperfective: false },
    ],
    "prěstavjati" => &[
        VerbDictionaryEntry { lemma: "prěstavjati", addition: "", transitive: true, imperfective: true },
    ],
    "prěstigati" => &[
        VerbDictionaryEntry { lemma: "prěstigati", addition: "", transitive: true, imperfective: true },
    ],
    "prěstignųti" => &[
        VerbDictionaryEntry { lemma: "prěstignųti", addition: "", transitive: true, imperfective: false },
    ],
    "prěstrašiti" => &[
        VerbDictionaryEntry { lemma: "prěstrašiti", addition: "", transitive: true, imperfective: false },
    ],
    "prěstųpati" => &[
        VerbDictionaryEntry { lemma: "prěstųpati", addition: "", transitive: true, imperfective: true },
    ],
    "prěstųpiti" => &[
        VerbDictionaryEntry { lemma: "prěstųpiti", addition: "", transitive: true, imperfective: false },
    ],
    "prěsunųti" => &[
        VerbDictionaryEntry { lemma: "prěsunųti", addition: "", transitive: true, imperfective: false },
    ],
    "prěsuvati" => &[
        VerbDictionaryEntry { lemma: "prěsuvati", addition: "", transitive: true, imperfective: true },
    ],
    "prěsvęzati" => &[
        VerbDictionaryEntry { lemma: "prěsvęzati", addition: "(prěsvęže)", transitive: true, imperfective: false },
    ],
    "prěsvęzyvati" => &[
        VerbDictionaryEntry { lemma: "prěsvęzyvati", addition: "", transitive: true, imperfective: true },
    ],
    "prěsědati" => &[
        VerbDictionaryEntry { lemma: "prěsědati", addition: "", transitive: true, imperfective: true },
    ],
    "prěsěkati" => &[
        VerbDictionaryEntry { lemma: "prěsěkati", addition: "", transitive: true, imperfective: true },
    ],
    "prěsěkti" => &[
        VerbDictionaryEntry { lemma: "prěsěkti", addition: "", transitive: true, imperfective: false },
    ],
    "prěsěsti" => &[
        VerbDictionaryEntry { lemma: "prěsěsti", addition: "(prěsěde)", transitive: true, imperfective: false },
    ],
    "prětekti" => &[
        VerbDictionaryEntry { lemma: "prětekti", addition: "", transitive: true, imperfective: false },
    ],
    "prěti" => &[
        VerbDictionaryEntry { lemma: "prěti", addition: "(pre)", transitive: false, imperfective: true },
    ],
    "prětraviti" => &[
        VerbDictionaryEntry { lemma: "prětraviti", addition: "", transitive: true, imperfective: false },
    ],
    "prětravjati" => &[
        VerbDictionaryEntry { lemma: "prětravjati", addition: "", transitive: true, imperfective: true },
    ],
    "prětvarjati" => &[
        VerbDictionaryEntry { lemma: "prětvarjati", addition: "", transitive: true, imperfective: true },
    ],
    "prětvoriti" => &[
        VerbDictionaryEntry { lemma: "prětvoriti", addition: "", transitive: true, imperfective: false },
    ],
    "prětěkati" => &[
        VerbDictionaryEntry { lemma: "prětěkati", addition: "", transitive: true, imperfective: true },
    ],
    "prětŕpěti" => &[
        VerbDictionaryEntry { lemma: "prětŕpěti", addition: "(prětŕpi)", transitive: true, imperfective: false },
    ],
    "prětȯlmačiti" => &[
        VerbDictionaryEntry { lemma: "prětȯlmačiti", addition: "", transitive: true, imperfective: false },
    ],
    "prěuveličati" => &[
        VerbDictionaryEntry { lemma: "prěuveličati", addition: "", transitive: true, imperfective: true },
    ],
    "prěuveličiti" => &[
        VerbDictionaryEntry { lemma: "prěuveličiti", addition: "", transitive: true, imperfective: false },
    ],
    "prěvariti" => &[
        VerbDictionaryEntry { lemma: "prěvariti", addition: "", transitive: true, imperfective: false },
    ],
    "prěvarjati" => &[
        VerbDictionaryEntry { lemma: "prěvarjati", addition: "", transitive: true, imperfective: true },
    ],
    "prěvažati" => &[
        VerbDictionaryEntry { lemma: "prěvažati", addition: "", transitive: true, imperfective: true },
    ],
    "prěvažiti" => &[
        VerbDictionaryEntry { lemma: "prěvažiti", addition: "", transitive: true, imperfective: false },
    ],
    "prěvesti" => &[
        VerbDictionaryEntry { lemma: "prěvesti", addition: "(prěvede)", transitive: true, imperfective: false },
    ],
    "prěvezti" => &[
        VerbDictionaryEntry { lemma: "prěvezti", addition: "", transitive: true, imperfective: false },
    ],
    "prěvladnųti" => &[
        VerbDictionaryEntry { lemma: "prěvladnųti", addition: "", transitive: false, imperfective: false },
    ],
    "prěvladyvati" => &[
        VerbDictionaryEntry { lemma: "prěvladyvati", addition: "", transitive: false, imperfective: true },
    ],
    "prěvoditi" => &[
        VerbDictionaryEntry { lemma: "prěvoditi", addition: "", transitive: true, imperfective: true },
    ],
    "prěvoziti" => &[
        VerbDictionaryEntry { lemma: "prěvoziti", addition: "", transitive: true, imperfective: true },
    ],
    "prěvysiti" => &[
        VerbDictionaryEntry { lemma: "prěvysiti", addition: "", transitive: true, imperfective: false },
    ],
    "prěvyšati" => &[
        VerbDictionaryEntry { lemma: "prěvyšati", addition: "", transitive: true, imperfective: true },
    ],
    "prěvȯzhoditi" => &[
        VerbDictionaryEntry { lemma: "prěvȯzhoditi", addition: "", transitive: true, imperfective: true },
    ],
    "prěvȯzȯjdti" => &[
        VerbDictionaryEntry { lemma: "prěvȯzȯjdti", addition: "(prěvȯzȯjde; prěvȯzšėl)", transitive: true, imperfective: false },
    ],
    "prězirati" => &[
        VerbDictionaryEntry { lemma: "prězirati", addition: "", transitive: true, imperfective: true },
    ],
    "prěznačiti" => &[
        VerbDictionaryEntry { lemma: "prěznačiti", addition: "", transitive: true, imperfective: false },
    ],
    "prězrěti" => &[
        VerbDictionaryEntry { lemma: "prězrěti", addition: "(prězri)", transitive: true, imperfective: false },
    ],
    "prězvati" => &[
        VerbDictionaryEntry { lemma: "prězvati", addition: "(prězȯve)", transitive: true, imperfective: false },
    ],
    "prězyvati" => &[
        VerbDictionaryEntry { lemma: "prězyvati", addition: "", transitive: true, imperfective: true },
    ],
    "prěčiti" => &[
        VerbDictionaryEntry { lemma: "prěčiti", addition: "", transitive: true, imperfective: true },
    ],
    "prěžiti" => &[
        VerbDictionaryEntry { lemma: "prěžiti", addition: "(prěžive)", transitive: true, imperfective: false },
    ],
    "prěživati" => &[
        VerbDictionaryEntry { lemma: "prěživati", addition: "", transitive: true, imperfective: true },
    ],
    "prěžuvati" => &[
        VerbDictionaryEntry { lemma: "prěžuvati", addition: "(prěžuje)", transitive: true, imperfective: false },
    ],
    "prųžiti" => &[
        VerbDictionaryEntry { lemma: "prųžiti", addition: "", transitive: true, imperfective: true },
    ],
    "psihovati" => &[
        VerbDictionaryEntry { lemma: "psihovati", addition: "", transitive: true, imperfective: true },
    ],
    "publikovati" => &[
        VerbDictionaryEntry { lemma: "publikovati", addition: "", transitive: true, imperfective: true },
    ],
    "puhnųti" => &[
        VerbDictionaryEntry { lemma: "puhnųti", addition: "", transitive: true, imperfective: true },
    ],
    "pustiti" => &[
        VerbDictionaryEntry { lemma: "pustiti", addition: "", transitive: true, imperfective: false },
    ],
    "pustošiti" => &[
        VerbDictionaryEntry { lemma: "pustošiti", addition: "", transitive: true, imperfective: true },
    ],
    "pustěti" => &[
        VerbDictionaryEntry { lemma: "pustěti", addition: "", transitive: true, imperfective: true },
    ],
    "pušiti" => &[
        VerbDictionaryEntry { lemma: "pušiti", addition: "", transitive: true, imperfective: true },
    ],
    "pušćati" => &[
        VerbDictionaryEntry { lemma: "pušćati", addition: "", transitive: true, imperfective: true },
    ],
    "pyliti" => &[
        VerbDictionaryEntry { lemma: "pyliti", addition: "", transitive: true, imperfective: true },
    ],
    "pytati" => &[
        VerbDictionaryEntry { lemma: "pytati", addition: "", transitive: true, imperfective: true },
    ],
    "pěvati" => &[
        VerbDictionaryEntry { lemma: "pěvati", addition: "", transitive: true, imperfective: true },
    ],
    "pŕděti" => &[
        VerbDictionaryEntry { lemma: "pŕděti", addition: "(pŕdi)", transitive: true, imperfective: true },
    ],
    "pŕti" => &[
        VerbDictionaryEntry { lemma: "pŕti", addition: "(pre)", transitive: false, imperfective: true },
    ],
    "pųditi" => &[
        VerbDictionaryEntry { lemma: "pųditi", addition: "", transitive: true, imperfective: true },
    ],
    "pųkati" => &[
        VerbDictionaryEntry { lemma: "pųkati", addition: "", transitive: true, imperfective: true },
    ],
    "pųknųti" => &[
        VerbDictionaryEntry { lemma: "pųknųti", addition: "", transitive: true, imperfective: false },
    ],
    "pųtovati" => &[
        VerbDictionaryEntry { lemma: "pųtovati", addition: "", transitive: true, imperfective: true },
    ],
    "pȯlniti" => &[
        VerbDictionaryEntry { lemma: "pȯlniti", addition: "", transitive: true, imperfective: true },
    ],
    "pȯlzati" => &[
        VerbDictionaryEntry { lemma: "pȯlzati", addition: "", transitive: true, imperfective: true },
    ],
    "pȯlzti" => &[
        VerbDictionaryEntry { lemma: "pȯlzti", addition: "", transitive: true, imperfective: true },
    ],
    "racionalizovati" => &[
        VerbDictionaryEntry { lemma: "racionalizovati", addition: "", transitive: true, imperfective: true },
    ],
    "raditi" => &[
        VerbDictionaryEntry { lemma: "raditi", addition: "", transitive: true, imperfective: true },
    ],
    "radovati" => &[
        VerbDictionaryEntry { lemma: "radovati", addition: "", transitive: true, imperfective: true },
    ],
    "raniti" => &[
        VerbDictionaryEntry { lemma: "raniti", addition: "", transitive: true, imperfective: true },
    ],
    "ratifikovati" => &[
        VerbDictionaryEntry { lemma: "ratifikovati", addition: "", transitive: true, imperfective: true },
    ],
    "raziti" => &[
        VerbDictionaryEntry { lemma: "raziti", addition: "", transitive: true, imperfective: true },
    ],
    "reagovati" => &[
        VerbDictionaryEntry { lemma: "reagovati", addition: "", transitive: true, imperfective: true },
    ],
    "realizovati" => &[
        VerbDictionaryEntry { lemma: "realizovati", addition: "", transitive: true, imperfective: true },
    ],
    "recitovati" => &[
        VerbDictionaryEntry { lemma: "recitovati", addition: "", transitive: true, imperfective: true },
    ],
    "redagovati" => &[
        VerbDictionaryEntry { lemma: "redagovati", addition: "", transitive: true, imperfective: true },
    ],
    "redaktovati" => &[
        VerbDictionaryEntry { lemma: "redaktovati", addition: "", transitive: true, imperfective: true },
    ],
    "redukovati" => &[
        VerbDictionaryEntry { lemma: "redukovati", addition: "", transitive: true, imperfective: true },
    ],
    "reformovati" => &[
        VerbDictionaryEntry { lemma: "reformovati", addition: "", transitive: true, imperfective: true },
    ],
    "regenerovati" => &[
        VerbDictionaryEntry { lemma: "regenerovati", addition: "", transitive: true, imperfective: true },
    ],
    "registrovati" => &[
        VerbDictionaryEntry { lemma: "registrovati", addition: "", transitive: true, imperfective: true },
    ],
    "regulovati" => &[
        VerbDictionaryEntry { lemma: "regulovati", addition: "", transitive: true, imperfective: true },
    ],
    "reklamovati" => &[
        VerbDictionaryEntry { lemma: "reklamovati", addition: "", transitive: true, imperfective: true },
    ],
    "rekomendovati" => &[
        VerbDictionaryEntry { lemma: "rekomendovati", addition: "", transitive: true, imperfective: true },
    ],
    "rekonstruovati" => &[
        VerbDictionaryEntry { lemma: "rekonstruovati", addition: "", transitive: true, imperfective: true },
    ],
    "remontovati" => &[
        VerbDictionaryEntry { lemma: "remontovati", addition: "", transitive: true, imperfective: true },
    ],
    "reorganizovati" => &[
        VerbDictionaryEntry { lemma: "reorganizovati", addition: "", transitive: true, imperfective: true },
    ],
    "reprezentovati" => &[
        VerbDictionaryEntry { lemma: "reprezentovati", addition: "", transitive: true, imperfective: true },
    ],
    "reprodukovati" => &[
        VerbDictionaryEntry { lemma: "reprodukovati", addition: "", transitive: true, imperfective: true },
    ],
    "restavrovati" => &[
        VerbDictionaryEntry { lemma: "restavrovati", addition: "", transitive: true, imperfective: true },
    ],
    "revidovati" => &[
        VerbDictionaryEntry { lemma: "revidovati", addition: "", transitive: true, imperfective: true },
    ],
    "revti" => &[
        VerbDictionaryEntry { lemma: "revti", addition: "", transitive: true, imperfective: true },
    ],
    "rezervovati" => &[
        VerbDictionaryEntry { lemma: "rezervovati", addition: "", transitive: true, imperfective: true },
    ],
    "rezumovati" => &[
        VerbDictionaryEntry { lemma: "rezumovati", addition: "", transitive: true, imperfective: true },
    ],
    "rizikovati" => &[
        VerbDictionaryEntry { lemma: "rizikovati", addition: "", transitive: true, imperfective: true },
    ],
    "roditi" => &[
        VerbDictionaryEntry { lemma: "roditi", addition: "", transitive: true, imperfective: true },
    ],
    "rokirovati" => &[
        VerbDictionaryEntry { lemma: "rokirovati", addition: "", transitive: true, imperfective: true },
    ],
    "roptati" => &[
        VerbDictionaryEntry { lemma: "roptati", addition: "", transitive: true, imperfective: true },
    ],
    "ruinovati" => &[
        VerbDictionaryEntry { lemma: "ruinovati", addition: "", transitive: true, imperfective: true },
    ],
    "ruměněti" => &[
        VerbDictionaryEntry { lemma: "ruměněti", addition: "", transitive: true, imperfective: true },
    ],
    "rusifikovati" => &[
        VerbDictionaryEntry { lemma: "rusifikovati", addition: "", transitive: true, imperfective: true },
    ],
    "rušiti" => &[
        VerbDictionaryEntry { lemma: "rušiti", addition: "", transitive: true, imperfective: true },
    ],
    "rvati" => &[
        VerbDictionaryEntry { lemma: "rvati", addition: "(rve)", transitive: true, imperfective: true },
    ],
    "rydati" => &[
        VerbDictionaryEntry { lemma: "rydati", addition: "", transitive: true, imperfective: true },
    ],
    "rygati" => &[
        VerbDictionaryEntry { lemma: "rygati", addition: "", transitive: true, imperfective: true },
    ],
    "rygnųti" => &[
        VerbDictionaryEntry { lemma: "rygnųti", addition: "", transitive: true, imperfective: false },
    ],
    "rymovati" => &[
        VerbDictionaryEntry { lemma: "rymovati", addition: "", transitive: true, imperfective: true },
    ],
    "rysovati" => &[
        VerbDictionaryEntry { lemma: "rysovati", addition: "", transitive: true, imperfective: true },
    ],
    "ryti" => &[
        VerbDictionaryEntry { lemma: "ryti", addition: "", transitive: true, imperfective: true },
    ],
    "råbiti" => &[
        VerbDictionaryEntry { lemma: "råbiti", addition: "", transitive: true, imperfective: true },
    ],
    "råbotati" => &[
        VerbDictionaryEntry { lemma: "råbotati", addition: "", transitive: true, imperfective: true },
    ],
    "råsti" => &[
        VerbDictionaryEntry { lemma: "råsti", addition: "(råste)", transitive: true, imperfective: true },
    ],
    "råvniti" => &[
        VerbDictionaryEntry { lemma: "råvniti", addition: "", transitive: true, imperfective: true },
    ],
    "råzbirati" => &[
        VerbDictionaryEntry { lemma: "råzbirati", addition: "", transitive: true, imperfective: true },
    ],
    "råzbiti" => &[
        VerbDictionaryEntry { lemma: "råzbiti", addition: "(råzbije)", transitive: true, imperfective: false },
    ],
    "råzbivati" => &[
        VerbDictionaryEntry { lemma: "råzbivati", addition: "", transitive: true, imperfective: true },
    ],
    "råzbrati" => &[
        VerbDictionaryEntry { lemma: "råzbrati", addition: "(råzbere)", transitive: true, imperfective: false },
    ],
    "råzbuditi" => &[
        VerbDictionaryEntry { lemma: "råzbuditi", addition: "", transitive: true, imperfective: false },
    ],
    "råzběsniti" => &[
        VerbDictionaryEntry { lemma: "råzběsniti", addition: "", transitive: true, imperfective: false },
    ],
    "råzběsnjati" => &[
        VerbDictionaryEntry { lemma: "råzběsnjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzcvěsti" => &[
        VerbDictionaryEntry { lemma: "råzcvěsti", addition: "(råzcvěte)", transitive: true, imperfective: false },
    ],
    "råzcvětati" => &[
        VerbDictionaryEntry { lemma: "råzcvětati", addition: "", transitive: true, imperfective: true },
    ],
    "råzdati" => &[
        VerbDictionaryEntry { lemma: "råzdati", addition: "(råzda)", transitive: true, imperfective: false },
    ],
    "råzdavati" => &[
        VerbDictionaryEntry { lemma: "råzdavati", addition: "", transitive: true, imperfective: true },
    ],
    "råzdirati" => &[
        VerbDictionaryEntry { lemma: "råzdirati", addition: "", transitive: true, imperfective: true },
    ],
    "råzdražniti" => &[
        VerbDictionaryEntry { lemma: "råzdražniti", addition: "", transitive: true, imperfective: false },
    ],
    "råzdražnjati" => &[
        VerbDictionaryEntry { lemma: "råzdražnjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzdrobiti" => &[
        VerbDictionaryEntry { lemma: "råzdrobiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzdrobjati" => &[
        VerbDictionaryEntry { lemma: "råzdrobjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzdrěti" => &[
        VerbDictionaryEntry { lemma: "råzdrěti", addition: "(råzdere)", transitive: true, imperfective: false },
    ],
    "råzdvajati" => &[
        VerbDictionaryEntry { lemma: "råzdvajati", addition: "", transitive: true, imperfective: true },
    ],
    "råzdvojiti" => &[
        VerbDictionaryEntry { lemma: "råzdvojiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzděliti" => &[
        VerbDictionaryEntry { lemma: "råzděliti", addition: "", transitive: true, imperfective: false },
    ],
    "råzděljati" => &[
        VerbDictionaryEntry { lemma: "råzděljati", addition: "", transitive: true, imperfective: true },
    ],
    "råzděti" => &[
        VerbDictionaryEntry { lemma: "råzděti", addition: "", transitive: true, imperfective: false },
    ],
    "råzděvati" => &[
        VerbDictionaryEntry { lemma: "råzděvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzdŕti" => &[
        VerbDictionaryEntry { lemma: "råzdŕti", addition: "(råzdere)", transitive: true, imperfective: false },
    ],
    "råzdųti" => &[
        VerbDictionaryEntry { lemma: "råzdųti", addition: "(råzdme)", transitive: true, imperfective: false },
    ],
    "råzdųvati" => &[
        VerbDictionaryEntry { lemma: "råzdųvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzdȯlbati" => &[
        VerbDictionaryEntry { lemma: "råzdȯlbati", addition: "", transitive: true, imperfective: true },
    ],
    "råzdȯlbti" => &[
        VerbDictionaryEntry { lemma: "råzdȯlbti", addition: "", transitive: true, imperfective: false },
    ],
    "råzgadati" => &[
        VerbDictionaryEntry { lemma: "råzgadati", addition: "", transitive: true, imperfective: false },
    ],
    "råzgadyvati" => &[
        VerbDictionaryEntry { lemma: "råzgadyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzganjati" => &[
        VerbDictionaryEntry { lemma: "råzganjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzgladiti" => &[
        VerbDictionaryEntry { lemma: "råzgladiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzglađati" => &[
        VerbDictionaryEntry { lemma: "råzglađati", addition: "", transitive: true, imperfective: true },
    ],
    "råzglašati" => &[
        VerbDictionaryEntry { lemma: "råzglašati", addition: "", transitive: true, imperfective: true },
    ],
    "råzglåsiti" => &[
        VerbDictionaryEntry { lemma: "råzglåsiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzględati" => &[
        VerbDictionaryEntry { lemma: "råzględati", addition: "", transitive: true, imperfective: true },
    ],
    "råzględěti" => &[
        VerbDictionaryEntry { lemma: "råzględěti", addition: "(råzględi)", transitive: true, imperfective: false },
    ],
    "råzgnati" => &[
        VerbDictionaryEntry { lemma: "råzgnati", addition: "(råzgone)", transitive: true, imperfective: false },
    ],
    "råzgněvati" => &[
        VerbDictionaryEntry { lemma: "råzgněvati", addition: "", transitive: true, imperfective: false },
    ],
    "råzgovarjati" => &[
        VerbDictionaryEntry { lemma: "råzgovarjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzgrabiti" => &[
        VerbDictionaryEntry { lemma: "råzgrabiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzgrabjati" => &[
        VerbDictionaryEntry { lemma: "råzgrabjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzgraničati" => &[
        VerbDictionaryEntry { lemma: "råzgraničati", addition: "", transitive: true, imperfective: true },
    ],
    "råzgraničiti" => &[
        VerbDictionaryEntry { lemma: "råzgraničiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzgrađati" => &[
        VerbDictionaryEntry { lemma: "råzgrađati", addition: "", transitive: true, imperfective: true },
    ],
    "råzgromiti" => &[
        VerbDictionaryEntry { lemma: "råzgromiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzgryzati" => &[
        VerbDictionaryEntry { lemma: "råzgryzati", addition: "", transitive: true, imperfective: true },
    ],
    "råzgryzti" => &[
        VerbDictionaryEntry { lemma: "råzgryzti", addition: "", transitive: true, imperfective: false },
    ],
    "råzgråditi" => &[
        VerbDictionaryEntry { lemma: "råzgråditi", addition: "", transitive: true, imperfective: false },
    ],
    "råzgrěti" => &[
        VerbDictionaryEntry { lemma: "råzgrěti", addition: "(råzgrěje)", transitive: true, imperfective: false },
    ],
    "råzgrěvati" => &[
        VerbDictionaryEntry { lemma: "råzgrěvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzigrati" => &[
        VerbDictionaryEntry { lemma: "råzigrati", addition: "", transitive: true, imperfective: false },
    ],
    "råzigryvati" => &[
        VerbDictionaryEntry { lemma: "råzigryvati", addition: "", transitive: true, imperfective: true },
    ],
    "råziskati" => &[
        VerbDictionaryEntry { lemma: "råziskati", addition: "", transitive: true, imperfective: false },
    ],
    "råziskyvati" => &[
        VerbDictionaryEntry { lemma: "råziskyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzjariti" => &[
        VerbDictionaryEntry { lemma: "råzjariti", addition: "", transitive: true, imperfective: false },
    ],
    "råzjarjati" => &[
        VerbDictionaryEntry { lemma: "råzjarjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzjasniti" => &[
        VerbDictionaryEntry { lemma: "råzjasniti", addition: "", transitive: true, imperfective: false },
    ],
    "råzjasnjati" => &[
        VerbDictionaryEntry { lemma: "råzjasnjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzjebati" => &[
        VerbDictionaryEntry { lemma: "råzjebati", addition: "(råzjebe)", transitive: true, imperfective: false },
    ],
    "råzjebyvati" => &[
        VerbDictionaryEntry { lemma: "råzjebyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzjedati" => &[
        VerbDictionaryEntry { lemma: "råzjedati", addition: "", transitive: true, imperfective: true },
    ],
    "råzjediniti" => &[
        VerbDictionaryEntry { lemma: "råzjediniti", addition: "", transitive: true, imperfective: false },
    ],
    "råzjedinjati" => &[
        VerbDictionaryEntry { lemma: "råzjedinjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzjesti" => &[
        VerbDictionaryEntry { lemma: "råzjesti", addition: "(råzje)", transitive: true, imperfective: false },
    ],
    "råzkalati" => &[
        VerbDictionaryEntry { lemma: "råzkalati", addition: "", transitive: true, imperfective: true },
    ],
    "råzkazati" => &[
        VerbDictionaryEntry { lemma: "råzkazati", addition: "(råzkaže)", transitive: true, imperfective: false },
    ],
    "råzkazyvati" => &[
        VerbDictionaryEntry { lemma: "råzkazyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzkladati" => &[
        VerbDictionaryEntry { lemma: "råzkladati", addition: "", transitive: true, imperfective: true },
    ],
    "råzklejati" => &[
        VerbDictionaryEntry { lemma: "råzklejati", addition: "", transitive: true, imperfective: true },
    ],
    "råzklejiti" => &[
        VerbDictionaryEntry { lemma: "råzklejiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzklåti" => &[
        VerbDictionaryEntry { lemma: "råzklåti", addition: "(råzkolje)", transitive: true, imperfective: false },
    ],
    "råzkodovati" => &[
        VerbDictionaryEntry { lemma: "råzkodovati", addition: "", transitive: true, imperfective: true },
    ],
    "råzkopati" => &[
        VerbDictionaryEntry { lemma: "råzkopati", addition: "", transitive: true, imperfective: false },
    ],
    "råzkopyvati" => &[
        VerbDictionaryEntry { lemma: "råzkopyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzkovati" => &[
        VerbDictionaryEntry { lemma: "råzkovati", addition: "", transitive: true, imperfective: false },
    ],
    "råzkovyvati" => &[
        VerbDictionaryEntry { lemma: "råzkovyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzkrajati" => &[
        VerbDictionaryEntry { lemma: "råzkrajati", addition: "", transitive: true, imperfective: true },
    ],
    "råzkrojiti" => &[
        VerbDictionaryEntry { lemma: "råzkrojiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzkryti" => &[
        VerbDictionaryEntry { lemma: "råzkryti", addition: "", transitive: true, imperfective: false },
    ],
    "råzkryvati" => &[
        VerbDictionaryEntry { lemma: "råzkryvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzkrųtiti" => &[
        VerbDictionaryEntry { lemma: "råzkrųtiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzkrųćati" => &[
        VerbDictionaryEntry { lemma: "råzkrųćati", addition: "", transitive: true, imperfective: true },
    ],
    "råzkrȯšiti" => &[
        VerbDictionaryEntry { lemma: "råzkrȯšiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzkvartirovati" => &[
        VerbDictionaryEntry { lemma: "råzkvartirovati", addition: "", transitive: true, imperfective: false },
    ],
    "råzkvartirovyvati" => &[
        VerbDictionaryEntry { lemma: "råzkvartirovyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzkydati" => &[
        VerbDictionaryEntry { lemma: "råzkydati", addition: "", transitive: true, imperfective: false },
    ],
    "råzkydyvati" => &[
        VerbDictionaryEntry { lemma: "råzkydyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzladiti" => &[
        VerbDictionaryEntry { lemma: "råzladiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzlagati" => &[
        VerbDictionaryEntry { lemma: "råzlagati", addition: "", transitive: true, imperfective: true },
    ],
    "råzlamyvati" => &[
        VerbDictionaryEntry { lemma: "råzlamyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzlađati" => &[
        VerbDictionaryEntry { lemma: "råzlađati", addition: "", transitive: true, imperfective: true },
    ],
    "råzliti" => &[
        VerbDictionaryEntry { lemma: "råzliti", addition: "(råzlije)", transitive: true, imperfective: false },
    ],
    "råzlivati" => &[
        VerbDictionaryEntry { lemma: "råzlivati", addition: "", transitive: true, imperfective: true },
    ],
    "råzličati" => &[
        VerbDictionaryEntry { lemma: "råzličati", addition: "", transitive: true, imperfective: true },
    ],
    "råzličiti" => &[
        VerbDictionaryEntry { lemma: "råzličiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzlomiti" => &[
        VerbDictionaryEntry { lemma: "råzlomiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzložiti" => &[
        VerbDictionaryEntry { lemma: "råzložiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzlųčati" => &[
        VerbDictionaryEntry { lemma: "råzlųčati", addition: "", transitive: true, imperfective: true },
    ],
    "råzlųčiti" => &[
        VerbDictionaryEntry { lemma: "råzlųčiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzmazati" => &[
        VerbDictionaryEntry { lemma: "råzmazati", addition: "(råzmaže)", transitive: true, imperfective: false },
    ],
    "råzmazyvati" => &[
        VerbDictionaryEntry { lemma: "råzmazyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzmesti" => &[
        VerbDictionaryEntry { lemma: "råzmesti", addition: "(råzmete)", transitive: true, imperfective: false },
    ],
    "råzmetati" => &[
        VerbDictionaryEntry { lemma: "råzmetati", addition: "", transitive: true, imperfective: false },
    ],
    "råzmetyvati" => &[
        VerbDictionaryEntry { lemma: "råzmetyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzmlåtiti" => &[
        VerbDictionaryEntry { lemma: "råzmlåtiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzmlěti" => &[
        VerbDictionaryEntry { lemma: "råzmlěti", addition: "(råzmelje)", transitive: true, imperfective: false },
    ],
    "råzmnažati" => &[
        VerbDictionaryEntry { lemma: "råzmnažati", addition: "", transitive: true, imperfective: true },
    ],
    "råzmnožiti" => &[
        VerbDictionaryEntry { lemma: "råzmnožiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzmokati" => &[
        VerbDictionaryEntry { lemma: "råzmokati", addition: "", transitive: true, imperfective: true },
    ],
    "råzmoknųti" => &[
        VerbDictionaryEntry { lemma: "råzmoknųti", addition: "", transitive: true, imperfective: false },
    ],
    "råzmontovati" => &[
        VerbDictionaryEntry { lemma: "råzmontovati", addition: "", transitive: true, imperfective: false },
    ],
    "råzmotati" => &[
        VerbDictionaryEntry { lemma: "råzmotati", addition: "", transitive: true, imperfective: false },
    ],
    "råzmražati" => &[
        VerbDictionaryEntry { lemma: "råzmražati", addition: "", transitive: true, imperfective: true },
    ],
    "råzmråziti" => &[
        VerbDictionaryEntry { lemma: "råzmråziti", addition: "", transitive: true, imperfective: false },
    ],
    "råzmysliti" => &[
        VerbDictionaryEntry { lemma: "råzmysliti", addition: "", transitive: true, imperfective: false },
    ],
    "råzmysljati" => &[
        VerbDictionaryEntry { lemma: "råzmysljati", addition: "", transitive: true, imperfective: true },
    ],
    "råzmyti" => &[
        VerbDictionaryEntry { lemma: "råzmyti", addition: "", transitive: true, imperfective: false },
    ],
    "råzmyvati" => &[
        VerbDictionaryEntry { lemma: "råzmyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzmękčati" => &[
        VerbDictionaryEntry { lemma: "råzmękčati", addition: "", transitive: true, imperfective: true },
    ],
    "råzmękčiti" => &[
        VerbDictionaryEntry { lemma: "råzmękčiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzmęti" => &[
        VerbDictionaryEntry { lemma: "råzmęti", addition: "(råzmne)", transitive: true, imperfective: false },
    ],
    "råzměniti" => &[
        VerbDictionaryEntry { lemma: "råzměniti", addition: "", transitive: true, imperfective: false },
    ],
    "råzměnjati" => &[
        VerbDictionaryEntry { lemma: "råzměnjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzměriti" => &[
        VerbDictionaryEntry { lemma: "råzměriti", addition: "", transitive: true, imperfective: false },
    ],
    "råzměrjati" => &[
        VerbDictionaryEntry { lemma: "råzměrjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzměstiti" => &[
        VerbDictionaryEntry { lemma: "råzměstiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzmětati" => &[
        VerbDictionaryEntry { lemma: "råzmětati", addition: "", transitive: true, imperfective: true },
    ],
    "råzměšati" => &[
        VerbDictionaryEntry { lemma: "råzměšati", addition: "", transitive: true, imperfective: false },
    ],
    "råzměšivati" => &[
        VerbDictionaryEntry { lemma: "råzměšivati", addition: "", transitive: true, imperfective: true },
    ],
    "råzměšćati" => &[
        VerbDictionaryEntry { lemma: "råzměšćati", addition: "", transitive: true, imperfective: true },
    ],
    "råznesti" => &[
        VerbDictionaryEntry { lemma: "råznesti", addition: "", transitive: true, imperfective: false },
    ],
    "råznositi" => &[
        VerbDictionaryEntry { lemma: "råznositi", addition: "", transitive: true, imperfective: true },
    ],
    "råzorati" => &[
        VerbDictionaryEntry { lemma: "råzorati", addition: "", transitive: true, imperfective: false },
    ],
    "råzoriti" => &[
        VerbDictionaryEntry { lemma: "råzoriti", addition: "", transitive: true, imperfective: false },
    ],
    "råzorjati" => &[
        VerbDictionaryEntry { lemma: "råzorjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzorųžati" => &[
        VerbDictionaryEntry { lemma: "råzorųžati", addition: "", transitive: true, imperfective: true },
    ],
    "råzorųžiti" => &[
        VerbDictionaryEntry { lemma: "råzorųžiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzočarovati" => &[
        VerbDictionaryEntry { lemma: "råzočarovati", addition: "", transitive: true, imperfective: false },
    ],
    "råzočarovyvati" => &[
        VerbDictionaryEntry { lemma: "råzočarovyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzpakovati" => &[
        VerbDictionaryEntry { lemma: "råzpakovati", addition: "", transitive: true, imperfective: false },
    ],
    "råzpakovyvati" => &[
        VerbDictionaryEntry { lemma: "råzpakovyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzpaliti" => &[
        VerbDictionaryEntry { lemma: "råzpaliti", addition: "", transitive: true, imperfective: false },
    ],
    "råzpaljati" => &[
        VerbDictionaryEntry { lemma: "råzpaljati", addition: "", transitive: true, imperfective: true },
    ],
    "råzparjati" => &[
        VerbDictionaryEntry { lemma: "råzparjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzpečatati" => &[
        VerbDictionaryEntry { lemma: "råzpečatati", addition: "", transitive: true, imperfective: false },
    ],
    "råzpečatyvati" => &[
        VerbDictionaryEntry { lemma: "råzpečatyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzpinati" => &[
        VerbDictionaryEntry { lemma: "råzpinati", addition: "(råzpne)", transitive: true, imperfective: true },
    ],
    "råzplesti" => &[
        VerbDictionaryEntry { lemma: "råzplesti", addition: "(råzplete)", transitive: true, imperfective: false },
    ],
    "råzpletati" => &[
        VerbDictionaryEntry { lemma: "råzpletati", addition: "", transitive: true, imperfective: true },
    ],
    "råzporęditi" => &[
        VerbDictionaryEntry { lemma: "råzporęditi", addition: "", transitive: true, imperfective: false },
    ],
    "råzporęđati" => &[
        VerbDictionaryEntry { lemma: "råzporęđati", addition: "", transitive: true, imperfective: true },
    ],
    "råzpoznati" => &[
        VerbDictionaryEntry { lemma: "råzpoznati", addition: "", transitive: true, imperfective: false },
    ],
    "råzpoznavati" => &[
        VerbDictionaryEntry { lemma: "råzpoznavati", addition: "", transitive: true, imperfective: true },
    ],
    "råzprašati" => &[
        VerbDictionaryEntry { lemma: "råzprašati", addition: "", transitive: true, imperfective: true },
    ],
    "råzprodati" => &[
        VerbDictionaryEntry { lemma: "råzprodati", addition: "(råzproda)", transitive: true, imperfective: false },
    ],
    "råzprodavati" => &[
        VerbDictionaryEntry { lemma: "råzprodavati", addition: "", transitive: true, imperfective: true },
    ],
    "råzprostirati" => &[
        VerbDictionaryEntry { lemma: "råzprostirati", addition: "", transitive: true, imperfective: true },
    ],
    "råzprostranjati" => &[
        VerbDictionaryEntry { lemma: "råzprostranjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzprostråniti" => &[
        VerbDictionaryEntry { lemma: "råzprostråniti", addition: "", transitive: true, imperfective: false },
    ],
    "råzprostrěti" => &[
        VerbDictionaryEntry { lemma: "råzprostrěti", addition: "(råzprostre)", transitive: true, imperfective: false },
    ],
    "råzprostŕti" => &[
        VerbDictionaryEntry { lemma: "råzprostŕti", addition: "(råzprostre)", transitive: true, imperfective: false },
    ],
    "råzpråti" => &[
        VerbDictionaryEntry { lemma: "råzpråti", addition: "(råzpoŕe)", transitive: true, imperfective: false },
    ],
    "råzpråšiti" => &[
        VerbDictionaryEntry { lemma: "råzpråšiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzpustiti" => &[
        VerbDictionaryEntry { lemma: "råzpustiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzpušćati" => &[
        VerbDictionaryEntry { lemma: "råzpušćati", addition: "", transitive: true, imperfective: true },
    ],
    "råzpytati" => &[
        VerbDictionaryEntry { lemma: "råzpytati", addition: "", transitive: true, imperfective: false },
    ],
    "råzpytyvati" => &[
        VerbDictionaryEntry { lemma: "råzpytyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzpęti" => &[
        VerbDictionaryEntry { lemma: "råzpęti", addition: "(råzpne)", transitive: true, imperfective: false },
    ],
    "råzpěniti" => &[
        VerbDictionaryEntry { lemma: "råzpěniti", addition: "", transitive: true, imperfective: false },
    ],
    "råzradovati" => &[
        VerbDictionaryEntry { lemma: "råzradovati", addition: "", transitive: true, imperfective: false },
    ],
    "råzraznjati" => &[
        VerbDictionaryEntry { lemma: "råzraznjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzrušati" => &[
        VerbDictionaryEntry { lemma: "råzrušati", addition: "", transitive: true, imperfective: true },
    ],
    "råzrušiti" => &[
        VerbDictionaryEntry { lemma: "råzrušiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzrvati" => &[
        VerbDictionaryEntry { lemma: "råzrvati", addition: "(råzrve)", transitive: true, imperfective: false },
    ],
    "råzryti" => &[
        VerbDictionaryEntry { lemma: "råzryti", addition: "", transitive: true, imperfective: false },
    ],
    "råzryvati" => &[
        VerbDictionaryEntry { lemma: "råzryvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzråbotati" => &[
        VerbDictionaryEntry { lemma: "råzråbotati", addition: "", transitive: true, imperfective: false },
    ],
    "råzråbotyvati" => &[
        VerbDictionaryEntry { lemma: "råzråbotyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzråzniti" => &[
        VerbDictionaryEntry { lemma: "råzråzniti", addition: "", transitive: true, imperfective: false },
    ],
    "råzrězati" => &[
        VerbDictionaryEntry { lemma: "råzrězati", addition: "(råzrěže)", transitive: true, imperfective: false },
    ],
    "råzrěšati" => &[
        VerbDictionaryEntry { lemma: "råzrěšati", addition: "", transitive: true, imperfective: true },
    ],
    "råzrěšiti" => &[
        VerbDictionaryEntry { lemma: "råzrěšiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzrųbati" => &[
        VerbDictionaryEntry { lemma: "råzrųbati", addition: "", transitive: true, imperfective: false },
    ],
    "råzrųbyvati" => &[
        VerbDictionaryEntry { lemma: "råzrųbyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzsaditi" => &[
        VerbDictionaryEntry { lemma: "råzsaditi", addition: "", transitive: true, imperfective: false },
    ],
    "råzsađati" => &[
        VerbDictionaryEntry { lemma: "råzsađati", addition: "", transitive: true, imperfective: true },
    ],
    "råzsedlati" => &[
        VerbDictionaryEntry { lemma: "råzsedlati", addition: "", transitive: true, imperfective: false },
    ],
    "råzsedlyvati" => &[
        VerbDictionaryEntry { lemma: "råzsedlyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzslati" => &[
        VerbDictionaryEntry { lemma: "råzslati", addition: "(råzšlje)", transitive: true, imperfective: false },
    ],
    "råzslěditi" => &[
        VerbDictionaryEntry { lemma: "råzslěditi", addition: "", transitive: true, imperfective: false },
    ],
    "råzslědovati" => &[
        VerbDictionaryEntry { lemma: "råzslědovati", addition: "", transitive: true, imperfective: true },
    ],
    "råzsmatrjati" => &[
        VerbDictionaryEntry { lemma: "råzsmatrjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzsmotriti" => &[
        VerbDictionaryEntry { lemma: "råzsmotriti", addition: "", transitive: true, imperfective: false },
    ],
    "råzsměšati" => &[
        VerbDictionaryEntry { lemma: "råzsměšati", addition: "", transitive: true, imperfective: true },
    ],
    "råzsměšiti" => &[
        VerbDictionaryEntry { lemma: "råzsměšiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzstaviti" => &[
        VerbDictionaryEntry { lemma: "råzstaviti", addition: "", transitive: true, imperfective: false },
    ],
    "råzstavjati" => &[
        VerbDictionaryEntry { lemma: "råzstavjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzsteliti" => &[
        VerbDictionaryEntry { lemma: "råzsteliti", addition: "", transitive: true, imperfective: false },
    ],
    "råzstrajati" => &[
        VerbDictionaryEntry { lemma: "råzstrajati", addition: "", transitive: true, imperfective: true },
    ],
    "råzstrojiti" => &[
        VerbDictionaryEntry { lemma: "råzstrojiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzstrěliti" => &[
        VerbDictionaryEntry { lemma: "råzstrěliti", addition: "", transitive: true, imperfective: false },
    ],
    "råzstrěljati" => &[
        VerbDictionaryEntry { lemma: "råzstrěljati", addition: "", transitive: true, imperfective: true },
    ],
    "råzsvirěpěti" => &[
        VerbDictionaryEntry { lemma: "råzsvirěpěti", addition: "", transitive: true, imperfective: false },
    ],
    "råzsylati" => &[
        VerbDictionaryEntry { lemma: "råzsylati", addition: "", transitive: true, imperfective: true },
    ],
    "råzsypati" => &[
        VerbDictionaryEntry { lemma: "råzsypati", addition: "", transitive: true, imperfective: false },
    ],
    "råzsypyvati" => &[
        VerbDictionaryEntry { lemma: "råzsypyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzsčitati" => &[
        VerbDictionaryEntry { lemma: "råzsčitati", addition: "", transitive: true, imperfective: false },
    ],
    "råzsčityvati" => &[
        VerbDictionaryEntry { lemma: "råzsčityvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzsějati" => &[
        VerbDictionaryEntry { lemma: "råzsějati", addition: "(råzsěje)", transitive: true, imperfective: false },
    ],
    "råzsějivati" => &[
        VerbDictionaryEntry { lemma: "råzsějivati", addition: "", transitive: true, imperfective: true },
    ],
    "råzsŕditi" => &[
        VerbDictionaryEntry { lemma: "råzsŕditi", addition: "", transitive: true, imperfective: false },
    ],
    "råzsųditi" => &[
        VerbDictionaryEntry { lemma: "råzsųditi", addition: "", transitive: true, imperfective: false },
    ],
    "råzsųđati" => &[
        VerbDictionaryEntry { lemma: "råzsųđati", addition: "", transitive: true, imperfective: true },
    ],
    "råztajati" => &[
        VerbDictionaryEntry { lemma: "råztajati", addition: "", transitive: true, imperfective: false },
    ],
    "råztapjati" => &[
        VerbDictionaryEntry { lemma: "råztapjati", addition: "", transitive: true, imperfective: true },
    ],
    "råztirati" => &[
        VerbDictionaryEntry { lemma: "råztirati", addition: "", transitive: true, imperfective: true },
    ],
    "råztopiti" => &[
        VerbDictionaryEntry { lemma: "råztopiti", addition: "", transitive: true, imperfective: false },
    ],
    "råztratiti" => &[
        VerbDictionaryEntry { lemma: "råztratiti", addition: "", transitive: true, imperfective: false },
    ],
    "råztraćati" => &[
        VerbDictionaryEntry { lemma: "råztraćati", addition: "", transitive: true, imperfective: true },
    ],
    "råztrgati" => &[
        VerbDictionaryEntry { lemma: "råztrgati", addition: "", transitive: true, imperfective: true },
    ],
    "råztrgnųti" => &[
        VerbDictionaryEntry { lemma: "råztrgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "råztrojiti" => &[
        VerbDictionaryEntry { lemma: "råztrojiti", addition: "", transitive: true, imperfective: false },
    ],
    "råztrěskati" => &[
        VerbDictionaryEntry { lemma: "råztrěskati", addition: "", transitive: true, imperfective: true },
    ],
    "råztrěsknųti" => &[
        VerbDictionaryEntry { lemma: "råztrěsknųti", addition: "", transitive: true, imperfective: false },
    ],
    "råztrěti" => &[
        VerbDictionaryEntry { lemma: "råztrěti", addition: "(råztre)", transitive: true, imperfective: false },
    ],
    "råztrųbiti" => &[
        VerbDictionaryEntry { lemma: "råztrųbiti", addition: "", transitive: true, imperfective: false },
    ],
    "råztvarjati" => &[
        VerbDictionaryEntry { lemma: "råztvarjati", addition: "", transitive: true, imperfective: true },
    ],
    "råztvoriti" => &[
        VerbDictionaryEntry { lemma: "råztvoriti", addition: "", transitive: true, imperfective: false },
    ],
    "råztęgati" => &[
        VerbDictionaryEntry { lemma: "råztęgati", addition: "", transitive: true, imperfective: true },
    ],
    "råztęgnųti" => &[
        VerbDictionaryEntry { lemma: "råztęgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "råztŕti" => &[
        VerbDictionaryEntry { lemma: "råztŕti", addition: "(råztre)", transitive: true, imperfective: false },
    ],
    "råztȯlkati" => &[
        VerbDictionaryEntry { lemma: "råztȯlkati", addition: "", transitive: true, imperfective: true },
    ],
    "råztȯlkti" => &[
        VerbDictionaryEntry { lemma: "råztȯlkti", addition: "", transitive: true, imperfective: false },
    ],
    "råztȯlstěti" => &[
        VerbDictionaryEntry { lemma: "råztȯlstěti", addition: "", transitive: true, imperfective: false },
    ],
    "råztȯptati" => &[
        VerbDictionaryEntry { lemma: "råztȯptati", addition: "", transitive: true, imperfective: false },
    ],
    "råztȯptyvati" => &[
        VerbDictionaryEntry { lemma: "råztȯptyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzumovati" => &[
        VerbDictionaryEntry { lemma: "råzumovati", addition: "", transitive: true, imperfective: true },
    ],
    "råzuměti" => &[
        VerbDictionaryEntry { lemma: "råzuměti", addition: "", transitive: true, imperfective: true },
    ],
    "råzvaliti" => &[
        VerbDictionaryEntry { lemma: "råzvaliti", addition: "", transitive: true, imperfective: false },
    ],
    "råzvaljati" => &[
        VerbDictionaryEntry { lemma: "råzvaljati", addition: "", transitive: true, imperfective: true },
    ],
    "råzvažati" => &[
        VerbDictionaryEntry { lemma: "råzvažati", addition: "", transitive: true, imperfective: true },
    ],
    "råzvažiti" => &[
        VerbDictionaryEntry { lemma: "råzvažiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzveseliti" => &[
        VerbDictionaryEntry { lemma: "råzveseliti", addition: "", transitive: true, imperfective: false },
    ],
    "råzveseljati" => &[
        VerbDictionaryEntry { lemma: "råzveseljati", addition: "", transitive: true, imperfective: true },
    ],
    "råzvesti" => &[
        VerbDictionaryEntry { lemma: "råzvesti", addition: "(råzvede)", transitive: true, imperfective: false },
        VerbDictionaryEntry { lemma: "råzvesti", addition: "", transitive: true, imperfective: false },
    ],
    "råzvezti" => &[
        VerbDictionaryEntry { lemma: "råzvezti", addition: "", transitive: true, imperfective: false },
    ],
    "råzviti" => &[
        VerbDictionaryEntry { lemma: "råzviti", addition: "(råzvije)", transitive: true, imperfective: false },
    ],
    "råzvivati" => &[
        VerbDictionaryEntry { lemma: "råzvivati", addition: "", transitive: true, imperfective: true },
    ],
    "råzvoditi" => &[
        VerbDictionaryEntry { lemma: "råzvoditi", addition: "", transitive: true, imperfective: true },
    ],
    "råzvoziti" => &[
        VerbDictionaryEntry { lemma: "råzvoziti", addition: "", transitive: true, imperfective: true },
    ],
    "råzvęzati" => &[
        VerbDictionaryEntry { lemma: "råzvęzati", addition: "(råzvęže)", transitive: true, imperfective: false },
    ],
    "råzvęzyvati" => &[
        VerbDictionaryEntry { lemma: "råzvęzyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzvědati" => &[
        VerbDictionaryEntry { lemma: "råzvědati", addition: "", transitive: true, imperfective: false },
    ],
    "råzvědyvati" => &[
        VerbDictionaryEntry { lemma: "råzvědyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzčesati" => &[
        VerbDictionaryEntry { lemma: "råzčesati", addition: "(råzčeše)", transitive: true, imperfective: false },
    ],
    "råzčistiti" => &[
        VerbDictionaryEntry { lemma: "råzčistiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzčišćati" => &[
        VerbDictionaryEntry { lemma: "råzčišćati", addition: "", transitive: true, imperfective: true },
    ],
    "råzčuliti" => &[
        VerbDictionaryEntry { lemma: "råzčuliti", addition: "", transitive: true, imperfective: false },
    ],
    "råzčuljati" => &[
        VerbDictionaryEntry { lemma: "råzčuljati", addition: "", transitive: true, imperfective: true },
    ],
    "råzšifrovati" => &[
        VerbDictionaryEntry { lemma: "råzšifrovati", addition: "", transitive: true, imperfective: true },
    ],
    "råzširiti" => &[
        VerbDictionaryEntry { lemma: "råzširiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzširjati" => &[
        VerbDictionaryEntry { lemma: "råzširjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzšnurovati" => &[
        VerbDictionaryEntry { lemma: "råzšnurovati", addition: "", transitive: true, imperfective: false },
    ],
    "råzšnurovyvati" => &[
        VerbDictionaryEntry { lemma: "råzšnurovyvati", addition: "", transitive: true, imperfective: true },
    ],
    "råzščepiti" => &[
        VerbDictionaryEntry { lemma: "råzščepiti", addition: "", transitive: true, imperfective: false },
    ],
    "råzščepjati" => &[
        VerbDictionaryEntry { lemma: "råzščepjati", addition: "", transitive: true, imperfective: true },
    ],
    "råzžegti" => &[
        VerbDictionaryEntry { lemma: "råzžegti", addition: "(råzžže)", transitive: true, imperfective: false },
    ],
    "råzžerati" => &[
        VerbDictionaryEntry { lemma: "råzžerati", addition: "", transitive: true, imperfective: true },
    ],
    "råzžigati" => &[
        VerbDictionaryEntry { lemma: "råzžigati", addition: "", transitive: true, imperfective: true },
    ],
    "råzžrěti" => &[
        VerbDictionaryEntry { lemma: "råzžrěti", addition: "", transitive: true, imperfective: false },
    ],
    "råzžuvati" => &[
        VerbDictionaryEntry { lemma: "råzžuvati", addition: "", transitive: true, imperfective: false },
    ],
    "rěkti" => &[
        VerbDictionaryEntry { lemma: "rěkti", addition: "", transitive: true, imperfective: true },
    ],
    "rězati" => &[
        VerbDictionaryEntry { lemma: "rězati", addition: "(rěže)", transitive: true, imperfective: true },
    ],
    "rěšati" => &[
        VerbDictionaryEntry { lemma: "rěšati", addition: "", transitive: true, imperfective: true },
    ],
    "rěšiti" => &[
        VerbDictionaryEntry { lemma: "rěšiti", addition: "", transitive: true, imperfective: false },
    ],
    "rųbati" => &[
        VerbDictionaryEntry { lemma: "rųbati", addition: "", transitive: true, imperfective: true },
    ],
    "rųkopleskati" => &[
        VerbDictionaryEntry { lemma: "rųkopleskati", addition: "", transitive: true, imperfective: true },
    ],
    "rųkoplesknųti" => &[
        VerbDictionaryEntry { lemma: "rųkoplesknųti", addition: "", transitive: true, imperfective: false },
    ],
    "rųkovati" => &[
        VerbDictionaryEntry { lemma: "rųkovati", addition: "", transitive: true, imperfective: true },
    ],
    "rųkovoditi" => &[
        VerbDictionaryEntry { lemma: "rųkovoditi", addition: "", transitive: true, imperfective: true },
    ],
    "sabotovati" => &[
        VerbDictionaryEntry { lemma: "sabotovati", addition: "", transitive: true, imperfective: true },
    ],
    "saditi" => &[
        VerbDictionaryEntry { lemma: "saditi", addition: "", transitive: true, imperfective: true },
    ],
    "samoubiti" => &[
        VerbDictionaryEntry { lemma: "samoubiti", addition: "", transitive: true, imperfective: false },
    ],
    "sbirati" => &[
        VerbDictionaryEntry { lemma: "sbirati", addition: "", transitive: true, imperfective: true },
    ],
    "sbudovati" => &[
        VerbDictionaryEntry { lemma: "sbudovati", addition: "", transitive: true, imperfective: false },
    ],
    "scati" => &[
        VerbDictionaryEntry { lemma: "scati", addition: "(sci)", transitive: true, imperfective: true },
    ],
    "sdeformovati" => &[
        VerbDictionaryEntry { lemma: "sdeformovati", addition: "", transitive: true, imperfective: false },
    ],
    "sdirati" => &[
        VerbDictionaryEntry { lemma: "sdirati", addition: "", transitive: true, imperfective: true },
    ],
    "sdrěmnųti" => &[
        VerbDictionaryEntry { lemma: "sdrěmnųti", addition: "", transitive: true, imperfective: false },
    ],
    "sdrěti" => &[
        VerbDictionaryEntry { lemma: "sdrěti", addition: "(sdre)", transitive: true, imperfective: false },
    ],
    "sdyhati" => &[
        VerbDictionaryEntry { lemma: "sdyhati", addition: "", transitive: true, imperfective: true },
    ],
    "sdělati" => &[
        VerbDictionaryEntry { lemma: "sdělati", addition: "", transitive: true, imperfective: false },
    ],
    "sdŕti" => &[
        VerbDictionaryEntry { lemma: "sdŕti", addition: "(sdre)", transitive: true, imperfective: false },
    ],
    "sdȯhnųti" => &[
        VerbDictionaryEntry { lemma: "sdȯhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "sebesmŕtiti" => &[
        VerbDictionaryEntry { lemma: "sebesmŕtiti", addition: "", transitive: true, imperfective: true },
    ],
    "sedlati" => &[
        VerbDictionaryEntry { lemma: "sedlati", addition: "", transitive: true, imperfective: true },
    ],
    "sgnesti" => &[
        VerbDictionaryEntry { lemma: "sgnesti", addition: "(sgnete)", transitive: true, imperfective: false },
    ],
    "sgnųti" => &[
        VerbDictionaryEntry { lemma: "sgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "sgrěšiti" => &[
        VerbDictionaryEntry { lemma: "sgrěšiti", addition: "", transitive: true, imperfective: false },
    ],
    "sgybati" => &[
        VerbDictionaryEntry { lemma: "sgybati", addition: "", transitive: true, imperfective: true },
    ],
    "shoditi" => &[
        VerbDictionaryEntry { lemma: "shoditi", addition: "", transitive: true, imperfective: true },
    ],
    "shovati" => &[
        VerbDictionaryEntry { lemma: "shovati", addition: "(shovaje)", transitive: true, imperfective: false },
    ],
    "shranjati" => &[
        VerbDictionaryEntry { lemma: "shranjati", addition: "", transitive: true, imperfective: true },
    ],
    "shråniti" => &[
        VerbDictionaryEntry { lemma: "shråniti", addition: "", transitive: true, imperfective: false },
    ],
    "shudnųti" => &[
        VerbDictionaryEntry { lemma: "shudnųti", addition: "", transitive: true, imperfective: false },
    ],
    "shvatiti" => &[
        VerbDictionaryEntry { lemma: "shvatiti", addition: "", transitive: true, imperfective: false },
    ],
    "signalizovati" => &[
        VerbDictionaryEntry { lemma: "signalizovati", addition: "", transitive: true, imperfective: true },
    ],
    "sikati" => &[
        VerbDictionaryEntry { lemma: "sikati", addition: "", transitive: true, imperfective: true },
    ],
    "simbolizovati" => &[
        VerbDictionaryEntry { lemma: "simbolizovati", addition: "", transitive: true, imperfective: true },
    ],
    "simulovati" => &[
        VerbDictionaryEntry { lemma: "simulovati", addition: "", transitive: true, imperfective: true },
    ],
    "sinhronizovati" => &[
        VerbDictionaryEntry { lemma: "sinhronizovati", addition: "", transitive: true, imperfective: true },
    ],
    "siněti" => &[
        VerbDictionaryEntry { lemma: "siněti", addition: "", transitive: true, imperfective: true },
    ],
    "sirotěti" => &[
        VerbDictionaryEntry { lemma: "sirotěti", addition: "", transitive: true, imperfective: true },
    ],
    "sivěti" => &[
        VerbDictionaryEntry { lemma: "sivěti", addition: "", transitive: true, imperfective: true },
    ],
    "sjedati" => &[
        VerbDictionaryEntry { lemma: "sjedati", addition: "", transitive: true, imperfective: true },
    ],
    "sjediniti" => &[
        VerbDictionaryEntry { lemma: "sjediniti", addition: "", transitive: true, imperfective: false },
    ],
    "sjedinjati" => &[
        VerbDictionaryEntry { lemma: "sjedinjati", addition: "", transitive: true, imperfective: true },
    ],
    "sjesti" => &[
        VerbDictionaryEntry { lemma: "sjesti", addition: "(sje)", transitive: true, imperfective: false },
    ],
    "sjęti" => &[
        VerbDictionaryEntry { lemma: "sjęti", addition: "(sȯjme)", transitive: true, imperfective: false },
    ],
    "skakati" => &[
        VerbDictionaryEntry { lemma: "skakati", addition: "", transitive: true, imperfective: true },
    ],
    "skanovati" => &[
        VerbDictionaryEntry { lemma: "skanovati", addition: "", transitive: true, imperfective: true },
    ],
    "skazati" => &[
        VerbDictionaryEntry { lemma: "skazati", addition: "(skaže)", transitive: true, imperfective: false },
    ],
    "skladati" => &[
        VerbDictionaryEntry { lemma: "skladati", addition: "", transitive: true, imperfective: true },
    ],
    "skladovati" => &[
        VerbDictionaryEntry { lemma: "skladovati", addition: "", transitive: true, imperfective: true },
    ],
    "sklanjati" => &[
        VerbDictionaryEntry { lemma: "sklanjati", addition: "", transitive: true, imperfective: true },
    ],
    "sklejiti" => &[
        VerbDictionaryEntry { lemma: "sklejiti", addition: "", transitive: true, imperfective: false },
    ],
    "skloniti" => &[
        VerbDictionaryEntry { lemma: "skloniti", addition: "", transitive: true, imperfective: false },
    ],
    "skombinovati" => &[
        VerbDictionaryEntry { lemma: "skombinovati", addition: "", transitive: true, imperfective: false },
    ],
    "skompensovati" => &[
        VerbDictionaryEntry { lemma: "skompensovati", addition: "", transitive: true, imperfective: false },
    ],
    "skomplikovati" => &[
        VerbDictionaryEntry { lemma: "skomplikovati", addition: "", transitive: true, imperfective: false },
    ],
    "skonfiskovati" => &[
        VerbDictionaryEntry { lemma: "skonfiskovati", addition: "", transitive: true, imperfective: false },
    ],
    "skončiti" => &[
        VerbDictionaryEntry { lemma: "skončiti", addition: "", transitive: true, imperfective: false },
    ],
    "skopiti" => &[
        VerbDictionaryEntry { lemma: "skopiti", addition: "", transitive: true, imperfective: true },
    ],
    "skočiti" => &[
        VerbDictionaryEntry { lemma: "skočiti", addition: "", transitive: true, imperfective: false },
    ],
    "skraćati" => &[
        VerbDictionaryEntry { lemma: "skraćati", addition: "", transitive: true, imperfective: true },
    ],
    "skrběti" => &[
        VerbDictionaryEntry { lemma: "skrběti", addition: "(skrbi)", transitive: true, imperfective: true },
    ],
    "skripnųti" => &[
        VerbDictionaryEntry { lemma: "skripnųti", addition: "", transitive: true, imperfective: false },
    ],
    "skripěti" => &[
        VerbDictionaryEntry { lemma: "skripěti", addition: "(skripi)", transitive: true, imperfective: true },
    ],
    "skriviti" => &[
        VerbDictionaryEntry { lemma: "skriviti", addition: "", transitive: true, imperfective: false },
    ],
    "skryti" => &[
        VerbDictionaryEntry { lemma: "skryti", addition: "", transitive: true, imperfective: false },
    ],
    "skryvati" => &[
        VerbDictionaryEntry { lemma: "skryvati", addition: "", transitive: true, imperfective: true },
    ],
    "skråtiti" => &[
        VerbDictionaryEntry { lemma: "skråtiti", addition: "", transitive: true, imperfective: false },
    ],
    "skubnųti" => &[
        VerbDictionaryEntry { lemma: "skubnųti", addition: "", transitive: true, imperfective: false },
    ],
    "skubti" => &[
        VerbDictionaryEntry { lemma: "skubti", addition: "", transitive: true, imperfective: true },
    ],
    "skuhati" => &[
        VerbDictionaryEntry { lemma: "skuhati", addition: "", transitive: true, imperfective: false },
    ],
    "skvrčati" => &[
        VerbDictionaryEntry { lemma: "skvrčati", addition: "(skvrči)", transitive: true, imperfective: true },
    ],
    "slaběti" => &[
        VerbDictionaryEntry { lemma: "slaběti", addition: "", transitive: true, imperfective: true },
    ],
    "slati" => &[
        VerbDictionaryEntry { lemma: "slati", addition: "(šlje)", transitive: true, imperfective: true },
    ],
    "slaviti" => &[
        VerbDictionaryEntry { lemma: "slaviti", addition: "", transitive: true, imperfective: true },
    ],
    "slgati" => &[
        VerbDictionaryEntry { lemma: "slgati", addition: "(slže)", transitive: true, imperfective: false },
    ],
    "sliti" => &[
        VerbDictionaryEntry { lemma: "sliti", addition: "(slije)", transitive: true, imperfective: false },
    ],
    "slivati" => &[
        VerbDictionaryEntry { lemma: "slivati", addition: "", transitive: true, imperfective: true },
    ],
    "slizgati" => &[
        VerbDictionaryEntry { lemma: "slizgati", addition: "", transitive: true, imperfective: true },
    ],
    "slizgnųti" => &[
        VerbDictionaryEntry { lemma: "slizgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "slomiti" => &[
        VerbDictionaryEntry { lemma: "slomiti", addition: "", transitive: true, imperfective: false },
    ],
    "slovjanizovati" => &[
        VerbDictionaryEntry { lemma: "slovjanizovati", addition: "", transitive: true, imperfective: true },
    ],
    "složiti" => &[
        VerbDictionaryEntry { lemma: "složiti", addition: "", transitive: true, imperfective: false },
    ],
    "slušati" => &[
        VerbDictionaryEntry { lemma: "slušati", addition: "", transitive: true, imperfective: true },
    ],
    "služiti" => &[
        VerbDictionaryEntry { lemma: "služiti", addition: "", transitive: true, imperfective: true },
    ],
    "slynųti" => &[
        VerbDictionaryEntry { lemma: "slynųti", addition: "", transitive: true, imperfective: true },
    ],
    "slyti" => &[
        VerbDictionaryEntry { lemma: "slyti", addition: "(slyne)", transitive: true, imperfective: true },
    ],
    "slyšati" => &[
        VerbDictionaryEntry { lemma: "slyšati", addition: "(slyši)", transitive: true, imperfective: true },
    ],
    "slåditi" => &[
        VerbDictionaryEntry { lemma: "slåditi", addition: "", transitive: true, imperfective: true },
    ],
    "slěditi" => &[
        VerbDictionaryEntry { lemma: "slěditi", addition: "", transitive: true, imperfective: true },
    ],
    "slěpnųti" => &[
        VerbDictionaryEntry { lemma: "slěpnųti", addition: "", transitive: true, imperfective: true },
    ],
    "slųčati" => &[
        VerbDictionaryEntry { lemma: "slųčati", addition: "", transitive: true, imperfective: true },
    ],
    "slųčiti" => &[
        VerbDictionaryEntry { lemma: "slųčiti", addition: "", transitive: true, imperfective: false },
    ],
    "smatrjati" => &[
        VerbDictionaryEntry { lemma: "smatrjati", addition: "", transitive: true, imperfective: true },
    ],
    "smažiti" => &[
        VerbDictionaryEntry { lemma: "smažiti", addition: "", transitive: true, imperfective: true },
    ],
    "smlåtiti" => &[
        VerbDictionaryEntry { lemma: "smlåtiti", addition: "", transitive: true, imperfective: false },
    ],
    "smlěti" => &[
        VerbDictionaryEntry { lemma: "smlěti", addition: "(smelje)", transitive: true, imperfective: false },
    ],
    "smogti" => &[
        VerbDictionaryEntry { lemma: "smogti", addition: "", transitive: false, imperfective: false },
    ],
    "smoliti" => &[
        VerbDictionaryEntry { lemma: "smoliti", addition: "", transitive: true, imperfective: true },
    ],
    "smrkati" => &[
        VerbDictionaryEntry { lemma: "smrkati", addition: "", transitive: true, imperfective: true },
    ],
    "smrščiti" => &[
        VerbDictionaryEntry { lemma: "smrščiti", addition: "", transitive: true, imperfective: false },
    ],
    "smućati" => &[
        VerbDictionaryEntry { lemma: "smućati", addition: "", transitive: true, imperfective: true },
    ],
    "smykati" => &[
        VerbDictionaryEntry { lemma: "smykati", addition: "", transitive: true, imperfective: true },
    ],
    "smėnšati" => &[
        VerbDictionaryEntry { lemma: "smėnšati", addition: "", transitive: true, imperfective: true },
    ],
    "smėnšiti" => &[
        VerbDictionaryEntry { lemma: "smėnšiti", addition: "", transitive: true, imperfective: false },
    ],
    "smękčati" => &[
        VerbDictionaryEntry { lemma: "smękčati", addition: "", transitive: true, imperfective: true },
    ],
    "smękčiti" => &[
        VerbDictionaryEntry { lemma: "smękčiti", addition: "", transitive: true, imperfective: false },
    ],
    "smęčkati" => &[
        VerbDictionaryEntry { lemma: "smęčkati", addition: "", transitive: true, imperfective: false },
    ],
    "směti" => &[
        VerbDictionaryEntry { lemma: "směti", addition: "", transitive: true, imperfective: true },
    ],
    "směšati" => &[
        VerbDictionaryEntry { lemma: "směšati", addition: "", transitive: true, imperfective: false },
    ],
    "směšivati" => &[
        VerbDictionaryEntry { lemma: "směšivati", addition: "", transitive: true, imperfective: true },
    ],
    "smŕditi" => &[
        VerbDictionaryEntry { lemma: "smŕditi", addition: "", transitive: true, imperfective: true },
    ],
    "smųtiti" => &[
        VerbDictionaryEntry { lemma: "smųtiti", addition: "", transitive: true, imperfective: false },
    ],
    "smųćati" => &[
        VerbDictionaryEntry { lemma: "smųćati", addition: "", transitive: true, imperfective: true },
    ],
    "snabděti" => &[
        VerbDictionaryEntry { lemma: "snabděti", addition: "(snabdi)", transitive: true, imperfective: false },
    ],
    "snabděvati" => &[
        VerbDictionaryEntry { lemma: "snabděvati", addition: "", transitive: true, imperfective: true },
    ],
    "snesti" => &[
        VerbDictionaryEntry { lemma: "snesti", addition: "", transitive: true, imperfective: false },
    ],
    "snetvarjati" => &[
        VerbDictionaryEntry { lemma: "snetvarjati", addition: "", transitive: true, imperfective: true },
    ],
    "snetvoriti" => &[
        VerbDictionaryEntry { lemma: "snetvoriti", addition: "", transitive: true, imperfective: false },
    ],
    "snimati" => &[
        VerbDictionaryEntry { lemma: "snimati", addition: "", transitive: true, imperfective: true },
    ],
    "sniti" => &[
        VerbDictionaryEntry { lemma: "sniti", addition: "", transitive: true, imperfective: true },
    ],
    "sniziti" => &[
        VerbDictionaryEntry { lemma: "sniziti", addition: "", transitive: true, imperfective: false },
    ],
    "snižati" => &[
        VerbDictionaryEntry { lemma: "snižati", addition: "", transitive: true, imperfective: true },
    ],
    "snositi" => &[
        VerbDictionaryEntry { lemma: "snositi", addition: "", transitive: true, imperfective: true },
    ],
    "snovati" => &[
        VerbDictionaryEntry { lemma: "snovati", addition: "", transitive: true, imperfective: true },
    ],
    "snědati" => &[
        VerbDictionaryEntry { lemma: "snědati", addition: "", transitive: true, imperfective: true },
    ],
    "sněžiti" => &[
        VerbDictionaryEntry { lemma: "sněžiti", addition: "", transitive: true, imperfective: true },
    ],
    "soliti" => &[
        VerbDictionaryEntry { lemma: "soliti", addition: "", transitive: true, imperfective: true },
    ],
    "sondovati" => &[
        VerbDictionaryEntry { lemma: "sondovati", addition: "", transitive: true, imperfective: true },
    ],
    "sortovati" => &[
        VerbDictionaryEntry { lemma: "sortovati", addition: "", transitive: true, imperfective: true },
    ],
    "spadati" => &[
        VerbDictionaryEntry { lemma: "spadati", addition: "", transitive: true, imperfective: true },
    ],
    "spakovati" => &[
        VerbDictionaryEntry { lemma: "spakovati", addition: "", transitive: true, imperfective: false },
    ],
    "spaliti" => &[
        VerbDictionaryEntry { lemma: "spaliti", addition: "", transitive: true, imperfective: false },
    ],
    "spasati" => &[
        VerbDictionaryEntry { lemma: "spasati", addition: "", transitive: true, imperfective: true },
    ],
    "spasti" => &[
        VerbDictionaryEntry { lemma: "spasti", addition: "(spade)", transitive: true, imperfective: false },
        VerbDictionaryEntry { lemma: "spasti", addition: "", transitive: true, imperfective: false },
    ],
    "spati" => &[
        VerbDictionaryEntry { lemma: "spati", addition: "(spi)", transitive: true, imperfective: true },
    ],
    "spekulovati" => &[
        VerbDictionaryEntry { lemma: "spekulovati", addition: "", transitive: true, imperfective: true },
    ],
    "spisyvati" => &[
        VerbDictionaryEntry { lemma: "spisyvati", addition: "", transitive: true, imperfective: true },
    ],
    "splesti" => &[
        VerbDictionaryEntry { lemma: "splesti", addition: "(splete)", transitive: true, imperfective: false },
    ],
    "sploditi" => &[
        VerbDictionaryEntry { lemma: "sploditi", addition: "", transitive: true, imperfective: false },
    ],
    "spolupracovati" => &[
        VerbDictionaryEntry { lemma: "spolupracovati", addition: "", transitive: true, imperfective: true },
    ],
    "spoluråbotyvati" => &[
        VerbDictionaryEntry { lemma: "spoluråbotyvati", addition: "", transitive: true, imperfective: true },
    ],
    "spomaliti" => &[
        VerbDictionaryEntry { lemma: "spomaliti", addition: "", transitive: true, imperfective: false },
    ],
    "spomaljati" => &[
        VerbDictionaryEntry { lemma: "spomaljati", addition: "", transitive: true, imperfective: true },
    ],
    "spominati" => &[
        VerbDictionaryEntry { lemma: "spominati", addition: "", transitive: true, imperfective: true },
    ],
    "spomněti" => &[
        VerbDictionaryEntry { lemma: "spomněti", addition: "(spomni)", transitive: true, imperfective: false },
    ],
    "sporiti" => &[
        VerbDictionaryEntry { lemma: "sporiti", addition: "", transitive: true, imperfective: true },
    ],
    "spotrěbiti" => &[
        VerbDictionaryEntry { lemma: "spotrěbiti", addition: "", transitive: true, imperfective: false },
    ],
    "spotrěbovati" => &[
        VerbDictionaryEntry { lemma: "spotrěbovati", addition: "", transitive: true, imperfective: true },
    ],
    "spotěti" => &[
        VerbDictionaryEntry { lemma: "spotěti", addition: "(spoti)", transitive: true, imperfective: false },
    ],
    "spoznati" => &[
        VerbDictionaryEntry { lemma: "spoznati", addition: "", transitive: true, imperfective: false },
    ],
    "spoznavati" => &[
        VerbDictionaryEntry { lemma: "spoznavati", addition: "", transitive: true, imperfective: true },
    ],
    "sprašati" => &[
        VerbDictionaryEntry { lemma: "sprašati", addition: "", transitive: true, imperfective: true },
    ],
    "sprositi" => &[
        VerbDictionaryEntry { lemma: "sprositi", addition: "", transitive: true, imperfective: false },
    ],
    "spręgati" => &[
        VerbDictionaryEntry { lemma: "spręgati", addition: "", transitive: true, imperfective: true },
    ],
    "spręsti" => &[
        VerbDictionaryEntry { lemma: "spręsti", addition: "(spręde)", transitive: true, imperfective: false },
    ],
    "spuhnųti" => &[
        VerbDictionaryEntry { lemma: "spuhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "spustiti" => &[
        VerbDictionaryEntry { lemma: "spustiti", addition: "", transitive: true, imperfective: false },
    ],
    "spušćati" => &[
        VerbDictionaryEntry { lemma: "spušćati", addition: "", transitive: true, imperfective: true },
    ],
    "spytati" => &[
        VerbDictionaryEntry { lemma: "spytati", addition: "", transitive: true, imperfective: false },
    ],
    "spěvati" => &[
        VerbDictionaryEntry { lemma: "spěvati", addition: "", transitive: true, imperfective: false },
    ],
    "spěšiti" => &[
        VerbDictionaryEntry { lemma: "spěšiti", addition: "", transitive: true, imperfective: true },
    ],
    "srati" => &[
        VerbDictionaryEntry { lemma: "srati", addition: "(sere)", transitive: true, imperfective: true },
    ],
    "sruinovati" => &[
        VerbDictionaryEntry { lemma: "sruinovati", addition: "", transitive: true, imperfective: false },
    ],
    "sråmiti" => &[
        VerbDictionaryEntry { lemma: "sråmiti", addition: "", transitive: true, imperfective: true },
    ],
    "sråvniti" => &[
        VerbDictionaryEntry { lemma: "sråvniti", addition: "", transitive: true, imperfective: false },
    ],
    "sråvnjati" => &[
        VerbDictionaryEntry { lemma: "sråvnjati", addition: "", transitive: true, imperfective: true },
    ],
    "sråzuměti" => &[
        VerbDictionaryEntry { lemma: "sråzuměti", addition: "", transitive: true, imperfective: false },
    ],
    "srųbati" => &[
        VerbDictionaryEntry { lemma: "srųbati", addition: "", transitive: true, imperfective: false },
    ],
    "srųbyvati" => &[
        VerbDictionaryEntry { lemma: "srųbyvati", addition: "", transitive: true, imperfective: true },
    ],
    "stabilizovati" => &[
        VerbDictionaryEntry { lemma: "stabilizovati", addition: "", transitive: true, imperfective: true },
    ],
    "standardizovati" => &[
        VerbDictionaryEntry { lemma: "standardizovati", addition: "", transitive: true, imperfective: true },
    ],
    "stanoviti" => &[
        VerbDictionaryEntry { lemma: "stanoviti", addition: "", transitive: true, imperfective: true },
    ],
    "stanųti" => &[
        VerbDictionaryEntry { lemma: "stanųti", addition: "", transitive: true, imperfective: false },
    ],
    "stapjati" => &[
        VerbDictionaryEntry { lemma: "stapjati", addition: "", transitive: true, imperfective: true },
    ],
    "startovati" => &[
        VerbDictionaryEntry { lemma: "startovati", addition: "", transitive: true, imperfective: true },
    ],
    "starěti" => &[
        VerbDictionaryEntry { lemma: "starěti", addition: "", transitive: true, imperfective: true },
    ],
    "stati" => &[
        VerbDictionaryEntry { lemma: "stati", addition: "(stane)", transitive: false, imperfective: false },
    ],
    "stavati" => &[
        VerbDictionaryEntry { lemma: "stavati", addition: "", transitive: false, imperfective: true },
    ],
    "staviti" => &[
        VerbDictionaryEntry { lemma: "staviti", addition: "", transitive: true, imperfective: true },
    ],
    "stavjati" => &[
        VerbDictionaryEntry { lemma: "stavjati", addition: "", transitive: true, imperfective: true },
    ],
    "stačiti" => &[
        VerbDictionaryEntry { lemma: "stačiti", addition: "", transitive: true, imperfective: true },
    ],
    "stekti" => &[
        VerbDictionaryEntry { lemma: "stekti", addition: "", transitive: true, imperfective: false },
    ],
    "steliti" => &[
        VerbDictionaryEntry { lemma: "steliti", addition: "", transitive: true, imperfective: true },
    ],
    "sterilizovati" => &[
        VerbDictionaryEntry { lemma: "sterilizovati", addition: "", transitive: true, imperfective: true },
    ],
    "stigati" => &[
        VerbDictionaryEntry { lemma: "stigati", addition: "", transitive: true, imperfective: true },
    ],
    "stignųti" => &[
        VerbDictionaryEntry { lemma: "stignųti", addition: "", transitive: true, imperfective: false },
    ],
    "stimulovati" => &[
        VerbDictionaryEntry { lemma: "stimulovati", addition: "", transitive: true, imperfective: true },
    ],
    "stiskati" => &[
        VerbDictionaryEntry { lemma: "stiskati", addition: "", transitive: true, imperfective: true },
    ],
    "stisknųti" => &[
        VerbDictionaryEntry { lemma: "stisknųti", addition: "", transitive: true, imperfective: false },
    ],
    "stlati" => &[
        VerbDictionaryEntry { lemma: "stlati", addition: "(stelje)", transitive: true, imperfective: true },
    ],
    "stojati" => &[
        VerbDictionaryEntry { lemma: "stojati", addition: "(stoji)", transitive: true, imperfective: true },
    ],
    "stonati" => &[
        VerbDictionaryEntry { lemma: "stonati", addition: "", transitive: true, imperfective: true },
    ],
    "stopiti" => &[
        VerbDictionaryEntry { lemma: "stopiti", addition: "", transitive: true, imperfective: false },
    ],
    "stradati" => &[
        VerbDictionaryEntry { lemma: "stradati", addition: "", transitive: true, imperfective: true },
    ],
    "strahovati" => &[
        VerbDictionaryEntry { lemma: "strahovati", addition: "", transitive: true, imperfective: true },
    ],
    "strajkovati" => &[
        VerbDictionaryEntry { lemma: "strajkovati", addition: "", transitive: true, imperfective: true },
    ],
    "strašiti" => &[
        VerbDictionaryEntry { lemma: "strašiti", addition: "", transitive: true, imperfective: true },
    ],
    "strigti" => &[
        VerbDictionaryEntry { lemma: "strigti", addition: "", transitive: true, imperfective: true },
    ],
    "strimati" => &[
        VerbDictionaryEntry { lemma: "strimati", addition: "", transitive: true, imperfective: false },
    ],
    "strimyvati" => &[
        VerbDictionaryEntry { lemma: "strimyvati", addition: "", transitive: true, imperfective: true },
    ],
    "strojiti" => &[
        VerbDictionaryEntry { lemma: "strojiti", addition: "", transitive: true, imperfective: true },
    ],
    "strugati" => &[
        VerbDictionaryEntry { lemma: "strugati", addition: "", transitive: true, imperfective: true },
    ],
    "strčiti" => &[
        VerbDictionaryEntry { lemma: "strčiti", addition: "", transitive: true, imperfective: true },
    ],
    "strěgti" => &[
        VerbDictionaryEntry { lemma: "strěgti", addition: "", transitive: true, imperfective: true },
    ],
    "strěliti" => &[
        VerbDictionaryEntry { lemma: "strěliti", addition: "", transitive: true, imperfective: true },
    ],
    "strěljati" => &[
        VerbDictionaryEntry { lemma: "strěljati", addition: "", transitive: true, imperfective: true },
    ],
    "strěsti" => &[
        VerbDictionaryEntry { lemma: "strěsti", addition: "(strěte)", transitive: true, imperfective: false },
    ],
    "strěćati" => &[
        VerbDictionaryEntry { lemma: "strěćati", addition: "", transitive: true, imperfective: true },
    ],
    "studiovati" => &[
        VerbDictionaryEntry { lemma: "studiovati", addition: "", transitive: true, imperfective: true },
    ],
    "studiti" => &[
        VerbDictionaryEntry { lemma: "studiti", addition: "", transitive: true, imperfective: true },
    ],
    "stukati" => &[
        VerbDictionaryEntry { lemma: "stukati", addition: "", transitive: true, imperfective: true },
    ],
    "stuknųti" => &[
        VerbDictionaryEntry { lemma: "stuknųti", addition: "", transitive: true, imperfective: false },
    ],
    "stvarjati" => &[
        VerbDictionaryEntry { lemma: "stvarjati", addition: "", transitive: true, imperfective: true },
    ],
    "stvoriti" => &[
        VerbDictionaryEntry { lemma: "stvoriti", addition: "", transitive: true, imperfective: false },
    ],
    "stųpati" => &[
        VerbDictionaryEntry { lemma: "stųpati", addition: "", transitive: true, imperfective: true },
    ],
    "stųpiti" => &[
        VerbDictionaryEntry { lemma: "stųpiti", addition: "", transitive: true, imperfective: false },
    ],
    "sunųti" => &[
        VerbDictionaryEntry { lemma: "sunųti", addition: "", transitive: true, imperfective: false },
    ],
    "surfovati" => &[
        VerbDictionaryEntry { lemma: "surfovati", addition: "", transitive: true, imperfective: true },
    ],
    "suvati" => &[
        VerbDictionaryEntry { lemma: "suvati", addition: "", transitive: true, imperfective: true },
    ],
    "sušiti" => &[
        VerbDictionaryEntry { lemma: "sušiti", addition: "", transitive: true, imperfective: true },
    ],
    "svabiti" => &[
        VerbDictionaryEntry { lemma: "svabiti", addition: "", transitive: true, imperfective: false },
    ],
    "svariti" => &[
        VerbDictionaryEntry { lemma: "svariti", addition: "", transitive: true, imperfective: false },
    ],
    "svarjati" => &[
        VerbDictionaryEntry { lemma: "svarjati", addition: "", transitive: true, imperfective: true },
    ],
    "svistati" => &[
        VerbDictionaryEntry { lemma: "svistati", addition: "", transitive: true, imperfective: true },
    ],
    "svistnųti" => &[
        VerbDictionaryEntry { lemma: "svistnųti", addition: "", transitive: true, imperfective: false },
    ],
    "sviti" => &[
        VerbDictionaryEntry { lemma: "sviti", addition: "(svije)", transitive: true, imperfective: false },
    ],
    "svętiti" => &[
        VerbDictionaryEntry { lemma: "svętiti", addition: "", transitive: true, imperfective: true },
    ],
    "svęzati" => &[
        VerbDictionaryEntry { lemma: "svęzati", addition: "(svęže)", transitive: true, imperfective: false },
    ],
    "svęzyvati" => &[
        VerbDictionaryEntry { lemma: "svęzyvati", addition: "", transitive: true, imperfective: true },
    ],
    "svědčiti" => &[
        VerbDictionaryEntry { lemma: "svědčiti", addition: "", transitive: true, imperfective: true },
    ],
    "světiti" => &[
        VerbDictionaryEntry { lemma: "světiti", addition: "", transitive: true, imperfective: true },
    ],
    "svŕběti" => &[
        VerbDictionaryEntry { lemma: "svŕběti", addition: "(svŕbi)", transitive: true, imperfective: true },
    ],
    "svŕgati" => &[
        VerbDictionaryEntry { lemma: "svŕgati", addition: "", transitive: true, imperfective: true },
    ],
    "svŕgnųti" => &[
        VerbDictionaryEntry { lemma: "svŕgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "sytiti" => &[
        VerbDictionaryEntry { lemma: "sytiti", addition: "", transitive: true, imperfective: true },
    ],
    "sčisliti" => &[
        VerbDictionaryEntry { lemma: "sčisliti", addition: "", transitive: true, imperfective: false },
    ],
    "sčitati" => &[
        VerbDictionaryEntry { lemma: "sčitati", addition: "", transitive: true, imperfective: false },
    ],
    "sęgati" => &[
        VerbDictionaryEntry { lemma: "sęgati", addition: "", transitive: true, imperfective: true },
    ],
    "sęgnųti" => &[
        VerbDictionaryEntry { lemma: "sęgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "sědati" => &[
        VerbDictionaryEntry { lemma: "sědati", addition: "", transitive: true, imperfective: true },
    ],
    "sědnųti" => &[
        VerbDictionaryEntry { lemma: "sědnųti", addition: "", transitive: true, imperfective: false },
    ],
    "sěděti" => &[
        VerbDictionaryEntry { lemma: "sěděti", addition: "(sědi)", transitive: true, imperfective: true },
    ],
    "sějati" => &[
        VerbDictionaryEntry { lemma: "sějati", addition: "(sěje)", transitive: true, imperfective: true },
    ],
    "sěkati" => &[
        VerbDictionaryEntry { lemma: "sěkati", addition: "", transitive: true, imperfective: true },
    ],
    "sěknųti" => &[
        VerbDictionaryEntry { lemma: "sěknųti", addition: "", transitive: true, imperfective: false },
    ],
    "sěkti" => &[
        VerbDictionaryEntry { lemma: "sěkti", addition: "", transitive: true, imperfective: true },
    ],
    "sěsti" => &[
        VerbDictionaryEntry { lemma: "sěsti", addition: "(sěde)", transitive: true, imperfective: false },
    ],
    "sŕditi" => &[
        VerbDictionaryEntry { lemma: "sŕditi", addition: "", transitive: true, imperfective: true },
    ],
    "sųditi" => &[
        VerbDictionaryEntry { lemma: "sųditi", addition: "", transitive: true, imperfective: true },
    ],
    "sųharmonizovati" => &[
        VerbDictionaryEntry { lemma: "sųharmonizovati", addition: "", transitive: true, imperfective: false },
    ],
    "sųpostaviti" => &[
        VerbDictionaryEntry { lemma: "sųpostaviti", addition: "", transitive: true, imperfective: false },
    ],
    "sųpostavjati" => &[
        VerbDictionaryEntry { lemma: "sųpostavjati", addition: "", transitive: true, imperfective: true },
    ],
    "sųprovađati" => &[
        VerbDictionaryEntry { lemma: "sųprovađati", addition: "", transitive: true, imperfective: true },
    ],
    "sųprovoditi" => &[
        VerbDictionaryEntry { lemma: "sųprovoditi", addition: "", transitive: true, imperfective: false },
    ],
    "sųråbotati" => &[
        VerbDictionaryEntry { lemma: "sųråbotati", addition: "", transitive: true, imperfective: true },
    ],
    "sųsrědotočati" => &[
        VerbDictionaryEntry { lemma: "sųsrědotočati", addition: "", transitive: true, imperfective: true },
    ],
    "sųsrědotočiti" => &[
        VerbDictionaryEntry { lemma: "sųsrědotočiti", addition: "", transitive: true, imperfective: false },
    ],
    "sųstrěsti" => &[
        VerbDictionaryEntry { lemma: "sųstrěsti", addition: "(sųstrěte)", transitive: true, imperfective: false },
    ],
    "sųstrěćati" => &[
        VerbDictionaryEntry { lemma: "sųstrěćati", addition: "", transitive: true, imperfective: true },
    ],
    "sųt" => &[
        VerbDictionaryEntry { lemma: "sųt", addition: "", transitive: false, imperfective: true },
    ],
    "sųtruditi" => &[
        VerbDictionaryEntry { lemma: "sųtruditi", addition: "", transitive: true, imperfective: true },
    ],
    "sųtstvovati" => &[
        VerbDictionaryEntry { lemma: "sųtstvovati", addition: "", transitive: true, imperfective: true },
    ],
    "sȯbrati" => &[
        VerbDictionaryEntry { lemma: "sȯbrati", addition: "(sbere)", transitive: true, imperfective: false },
    ],
    "sȯdŕžati" => &[
        VerbDictionaryEntry { lemma: "sȯdŕžati", addition: "", transitive: true, imperfective: false },
    ],
    "sȯdŕživati" => &[
        VerbDictionaryEntry { lemma: "sȯdŕživati", addition: "", transitive: true, imperfective: true },
    ],
    "sȯhnųti" => &[
        VerbDictionaryEntry { lemma: "sȯhnųti", addition: "", transitive: true, imperfective: true },
    ],
    "sȯjdti" => &[
        VerbDictionaryEntry { lemma: "sȯjdti", addition: "(sȯjde; sȯšėl)", transitive: true, imperfective: false },
    ],
    "sȯmknųti" => &[
        VerbDictionaryEntry { lemma: "sȯmknųti", addition: "", transitive: true, imperfective: false },
    ],
    "sȯobćati" => &[
        VerbDictionaryEntry { lemma: "sȯobćati", addition: "", transitive: true, imperfective: true },
    ],
    "sȯobćiti" => &[
        VerbDictionaryEntry { lemma: "sȯobćiti", addition: "", transitive: true, imperfective: false },
    ],
    "sȯsati" => &[
        VerbDictionaryEntry { lemma: "sȯsati", addition: "", transitive: true, imperfective: true },
    ],
    "sȯstaviti" => &[
        VerbDictionaryEntry { lemma: "sȯstaviti", addition: "", transitive: true, imperfective: false },
    ],
    "sȯstavjati" => &[
        VerbDictionaryEntry { lemma: "sȯstavjati", addition: "", transitive: true, imperfective: true },
    ],
    "sȯtkati" => &[
        VerbDictionaryEntry { lemma: "sȯtkati", addition: "", transitive: true, imperfective: false },
    ],
    "sȯvladnųti" => &[
        VerbDictionaryEntry { lemma: "sȯvladnųti", addition: "", transitive: true, imperfective: false },
    ],
    "sȯvladyvati" => &[
        VerbDictionaryEntry { lemma: "sȯvladyvati", addition: "", transitive: true, imperfective: true },
    ],
    "sȯvpadati" => &[
        VerbDictionaryEntry { lemma: "sȯvpadati", addition: "", transitive: true, imperfective: true },
    ],
    "sȯvětovati" => &[
        VerbDictionaryEntry { lemma: "sȯvětovati", addition: "", transitive: true, imperfective: true },
    ],
    "sȯvŕšati" => &[
        VerbDictionaryEntry { lemma: "sȯvŕšati", addition: "", transitive: true, imperfective: true },
    ],
    "sȯvŕšiti" => &[
        VerbDictionaryEntry { lemma: "sȯvŕšiti", addition: "", transitive: true, imperfective: false },
    ],
    "sȯzdati" => &[
        VerbDictionaryEntry { lemma: "sȯzdati", addition: "", transitive: true, imperfective: false },
    ],
    "sȯzdavati" => &[
        VerbDictionaryEntry { lemma: "sȯzdavati", addition: "", transitive: true, imperfective: true },
    ],
    "sȯzvati" => &[
        VerbDictionaryEntry { lemma: "sȯzvati", addition: "(sȯzȯve)", transitive: true, imperfective: false },
    ],
    "sȯzyvati" => &[
        VerbDictionaryEntry { lemma: "sȯzyvati", addition: "", transitive: true, imperfective: true },
    ],
    "sȯčuvstvovati" => &[
        VerbDictionaryEntry { lemma: "sȯčuvstvovati", addition: "", transitive: true, imperfective: true },
    ],
    "sȯžrti" => &[
        VerbDictionaryEntry { lemma: "sȯžrti", addition: "(sȯžre)", transitive: true, imperfective: false },
    ],
    "sȯžrěti" => &[
        VerbDictionaryEntry { lemma: "sȯžrěti", addition: "(sȯžre)", transitive: true, imperfective: false },
    ],
    "tajati" => &[
        VerbDictionaryEntry { lemma: "tajati", addition: "", transitive: true, imperfective: true },
    ],
    "tajiti" => &[
        VerbDictionaryEntry { lemma: "tajiti", addition: "", transitive: true, imperfective: true },
    ],
    "tancevati" => &[
        VerbDictionaryEntry { lemma: "tancevati", addition: "", transitive: true, imperfective: true },
    ],
    "tatuovati" => &[
        VerbDictionaryEntry { lemma: "tatuovati", addition: "", transitive: true, imperfective: true },
    ],
    "tekti" => &[
        VerbDictionaryEntry { lemma: "tekti", addition: "", transitive: true, imperfective: true },
    ],
    "telefonovati" => &[
        VerbDictionaryEntry { lemma: "telefonovati", addition: "", transitive: true, imperfective: true },
    ],
    "tesati" => &[
        VerbDictionaryEntry { lemma: "tesati", addition: "(teše)", transitive: true, imperfective: true },
    ],
    "testovati" => &[
        VerbDictionaryEntry { lemma: "testovati", addition: "", transitive: true, imperfective: true },
    ],
    "tipkati" => &[
        VerbDictionaryEntry { lemma: "tipkati", addition: "", transitive: true, imperfective: true },
    ],
    "tiskati" => &[
        VerbDictionaryEntry { lemma: "tiskati", addition: "", transitive: true, imperfective: true },
    ],
    "tisknųti" => &[
        VerbDictionaryEntry { lemma: "tisknųti", addition: "", transitive: true, imperfective: false },
    ],
    "tkati" => &[
        VerbDictionaryEntry { lemma: "tkati", addition: "", transitive: true, imperfective: true },
    ],
    "tknųti" => &[
        VerbDictionaryEntry { lemma: "tknųti", addition: "", transitive: true, imperfective: false },
    ],
    "tlěti" => &[
        VerbDictionaryEntry { lemma: "tlěti", addition: "", transitive: true, imperfective: true },
    ],
    "tonųti" => &[
        VerbDictionaryEntry { lemma: "tonųti", addition: "", transitive: true, imperfective: true },
    ],
    "topiti" => &[
        VerbDictionaryEntry { lemma: "topiti", addition: "", transitive: true, imperfective: true },
    ],
    "točiti" => &[
        VerbDictionaryEntry { lemma: "točiti", addition: "", transitive: true, imperfective: true },
    ],
    "trajati" => &[
        VerbDictionaryEntry { lemma: "trajati", addition: "", transitive: true, imperfective: true },
    ],
    "transkribovati" => &[
        VerbDictionaryEntry { lemma: "transkribovati", addition: "", transitive: true, imperfective: true },
    ],
    "transliterovati" => &[
        VerbDictionaryEntry { lemma: "transliterovati", addition: "", transitive: true, imperfective: true },
    ],
    "transportovati" => &[
        VerbDictionaryEntry { lemma: "transportovati", addition: "", transitive: true, imperfective: true },
    ],
    "tratiti" => &[
        VerbDictionaryEntry { lemma: "tratiti", addition: "", transitive: true, imperfective: true },
    ],
    "travmatizovati" => &[
        VerbDictionaryEntry { lemma: "travmatizovati", addition: "", transitive: true, imperfective: true },
    ],
    "trenovati" => &[
        VerbDictionaryEntry { lemma: "trenovati", addition: "", transitive: true, imperfective: true },
    ],
    "trepetati" => &[
        VerbDictionaryEntry { lemma: "trepetati", addition: "", transitive: true, imperfective: true },
    ],
    "trgati" => &[
        VerbDictionaryEntry { lemma: "trgati", addition: "", transitive: true, imperfective: true },
    ],
    "trgnųti" => &[
        VerbDictionaryEntry { lemma: "trgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "trgovati" => &[
        VerbDictionaryEntry { lemma: "trgovati", addition: "", transitive: true, imperfective: true },
    ],
    "trimati" => &[
        VerbDictionaryEntry { lemma: "trimati", addition: "", transitive: true, imperfective: true },
    ],
    "trvati" => &[
        VerbDictionaryEntry { lemma: "trvati", addition: "", transitive: true, imperfective: true },
    ],
    "trėvožiti" => &[
        VerbDictionaryEntry { lemma: "trėvožiti", addition: "", transitive: true, imperfective: true },
    ],
    "tręsti" => &[
        VerbDictionaryEntry { lemma: "tręsti", addition: "", transitive: true, imperfective: true },
    ],
    "trěbovati" => &[
        VerbDictionaryEntry { lemma: "trěbovati", addition: "", transitive: false, imperfective: true },
    ],
    "trěti" => &[
        VerbDictionaryEntry { lemma: "trěti", addition: "(tre)", transitive: true, imperfective: true },
    ],
    "trězvěti" => &[
        VerbDictionaryEntry { lemma: "trězvěti", addition: "", transitive: true, imperfective: true },
    ],
    "trųbiti" => &[
        VerbDictionaryEntry { lemma: "trųbiti", addition: "", transitive: true, imperfective: true },
    ],
    "tvoriti" => &[
        VerbDictionaryEntry { lemma: "tvoriti", addition: "", transitive: true, imperfective: true },
    ],
    "tvŕditi" => &[
        VerbDictionaryEntry { lemma: "tvŕditi", addition: "", transitive: true, imperfective: true },
    ],
    "tvŕdnųti" => &[
        VerbDictionaryEntry { lemma: "tvŕdnųti", addition: "", transitive: true, imperfective: true },
    ],
    "tvŕděti" => &[
        VerbDictionaryEntry { lemma: "tvŕděti", addition: "", transitive: true, imperfective: true },
    ],
    "tykati" => &[
        VerbDictionaryEntry { lemma: "tykati", addition: "(tyče)", transitive: true, imperfective: true },
        VerbDictionaryEntry { lemma: "tykati", addition: "", transitive: true, imperfective: true },
    ],
    "tėmněti" => &[
        VerbDictionaryEntry { lemma: "tėmněti", addition: "", transitive: true, imperfective: true },
    ],
    "tęgati" => &[
        VerbDictionaryEntry { lemma: "tęgati", addition: "", transitive: true, imperfective: true },
    ],
    "tęgnųti" => &[
        VerbDictionaryEntry { lemma: "tęgnųti", addition: "", transitive: true, imperfective: true },
    ],
    "tŕpěti" => &[
        VerbDictionaryEntry { lemma: "tŕpěti", addition: "(tŕpi)", transitive: true, imperfective: true },
    ],
    "tŕti" => &[
        VerbDictionaryEntry { lemma: "tŕti", addition: "(tre)", transitive: true, imperfective: true },
    ],
    "tųpěti" => &[
        VerbDictionaryEntry { lemma: "tųpěti", addition: "", transitive: true, imperfective: true },
    ],
    "tųžiti" => &[
        VerbDictionaryEntry { lemma: "tųžiti", addition: "", transitive: true, imperfective: true },
    ],
    "tȯlkti" => &[
        VerbDictionaryEntry { lemma: "tȯlkti", addition: "", transitive: true, imperfective: true },
    ],
    "tȯlmačiti" => &[
        VerbDictionaryEntry { lemma: "tȯlmačiti", addition: "", transitive: true, imperfective: true },
    ],
    "ubiti" => &[
        VerbDictionaryEntry { lemma: "ubiti", addition: "(ubije)", transitive: true, imperfective: false },
    ],
    "ubivati" => &[
        VerbDictionaryEntry { lemma: "ubivati", addition: "", transitive: true, imperfective: true },
    ],
    "uběditi" => &[
        VerbDictionaryEntry { lemma: "uběditi", addition: "", transitive: true, imperfective: false },
    ],
    "uběgati" => &[
        VerbDictionaryEntry { lemma: "uběgati", addition: "", transitive: true, imperfective: true },
    ],
    "uběgti" => &[
        VerbDictionaryEntry { lemma: "uběgti", addition: "(uběži)", transitive: true, imperfective: false },
    ],
    "uběđati" => &[
        VerbDictionaryEntry { lemma: "uběđati", addition: "", transitive: true, imperfective: true },
    ],
    "udaliti" => &[
        VerbDictionaryEntry { lemma: "udaliti", addition: "", transitive: true, imperfective: false },
    ],
    "udaljati" => &[
        VerbDictionaryEntry { lemma: "udaljati", addition: "", transitive: true, imperfective: true },
    ],
    "udariti" => &[
        VerbDictionaryEntry { lemma: "udariti", addition: "", transitive: true, imperfective: false },
    ],
    "udarjati" => &[
        VerbDictionaryEntry { lemma: "udarjati", addition: "", transitive: true, imperfective: true },
    ],
    "udaviti" => &[
        VerbDictionaryEntry { lemma: "udaviti", addition: "", transitive: true, imperfective: false },
    ],
    "udiviti" => &[
        VerbDictionaryEntry { lemma: "udiviti", addition: "", transitive: true, imperfective: false },
    ],
    "udivjati" => &[
        VerbDictionaryEntry { lemma: "udivjati", addition: "", transitive: true, imperfective: true },
    ],
    "udoskonaliti" => &[
        VerbDictionaryEntry { lemma: "udoskonaliti", addition: "", transitive: true, imperfective: false },
    ],
    "udušiti" => &[
        VerbDictionaryEntry { lemma: "udušiti", addition: "", transitive: true, imperfective: false },
    ],
    "udvojiti" => &[
        VerbDictionaryEntry { lemma: "udvojiti", addition: "", transitive: true, imperfective: false },
    ],
    "uděliti" => &[
        VerbDictionaryEntry { lemma: "uděliti", addition: "", transitive: true, imperfective: false },
    ],
    "uděljati" => &[
        VerbDictionaryEntry { lemma: "uděljati", addition: "", transitive: true, imperfective: true },
    ],
    "udŕžati" => &[
        VerbDictionaryEntry { lemma: "udŕžati", addition: "", transitive: true, imperfective: false },
    ],
    "udŕživati" => &[
        VerbDictionaryEntry { lemma: "udŕživati", addition: "", transitive: true, imperfective: true },
    ],
    "ugadati" => &[
        VerbDictionaryEntry { lemma: "ugadati", addition: "", transitive: true, imperfective: false },
    ],
    "ugadyvati" => &[
        VerbDictionaryEntry { lemma: "ugadyvati", addition: "", transitive: true, imperfective: true },
    ],
    "ugasati" => &[
        VerbDictionaryEntry { lemma: "ugasati", addition: "", transitive: true, imperfective: true },
    ],
    "ugasnųti" => &[
        VerbDictionaryEntry { lemma: "ugasnųti", addition: "", transitive: true, imperfective: false },
    ],
    "uględati" => &[
        VerbDictionaryEntry { lemma: "uględati", addition: "", transitive: true, imperfective: true },
    ],
    "uględěti" => &[
        VerbDictionaryEntry { lemma: "uględěti", addition: "(uględi)", transitive: true, imperfective: false },
    ],
    "ugryzti" => &[
        VerbDictionaryEntry { lemma: "ugryzti", addition: "", transitive: true, imperfective: false },
    ],
    "uhoditi" => &[
        VerbDictionaryEntry { lemma: "uhoditi", addition: "", transitive: true, imperfective: true },
    ],
    "ujdti" => &[
        VerbDictionaryEntry { lemma: "ujdti", addition: "(ujde; ušėl)", transitive: true, imperfective: false },
    ],
    "ujediniti" => &[
        VerbDictionaryEntry { lemma: "ujediniti", addition: "", transitive: true, imperfective: false },
    ],
    "ujedinjati" => &[
        VerbDictionaryEntry { lemma: "ujedinjati", addition: "", transitive: true, imperfective: true },
    ],
    "ujehati" => &[
        VerbDictionaryEntry { lemma: "ujehati", addition: "(ujede)", transitive: true, imperfective: false },
    ],
    "uježđati" => &[
        VerbDictionaryEntry { lemma: "uježđati", addition: "", transitive: true, imperfective: true },
    ],
    "ujmati" => &[
        VerbDictionaryEntry { lemma: "ujmati", addition: "", transitive: true, imperfective: true },
    ],
    "ujęti" => &[
        VerbDictionaryEntry { lemma: "ujęti", addition: "(ujme)", transitive: true, imperfective: false },
    ],
    "ukazati" => &[
        VerbDictionaryEntry { lemma: "ukazati", addition: "(ukaže)", transitive: true, imperfective: false },
    ],
    "ukazyvati" => &[
        VerbDictionaryEntry { lemma: "ukazyvati", addition: "", transitive: true, imperfective: true },
    ],
    "uklåti" => &[
        VerbDictionaryEntry { lemma: "uklåti", addition: "(ukolje)", transitive: true, imperfective: false },
    ],
    "ukrasiti" => &[
        VerbDictionaryEntry { lemma: "ukrasiti", addition: "", transitive: true, imperfective: false },
    ],
    "ukrasti" => &[
        VerbDictionaryEntry { lemma: "ukrasti", addition: "(ukrade)", transitive: true, imperfective: false },
    ],
    "ukrašati" => &[
        VerbDictionaryEntry { lemma: "ukrašati", addition: "", transitive: true, imperfective: true },
    ],
    "ukrotiti" => &[
        VerbDictionaryEntry { lemma: "ukrotiti", addition: "", transitive: true, imperfective: false },
    ],
    "ukryti" => &[
        VerbDictionaryEntry { lemma: "ukryti", addition: "", transitive: true, imperfective: false },
    ],
    "ukryvati" => &[
        VerbDictionaryEntry { lemma: "ukryvati", addition: "", transitive: true, imperfective: true },
    ],
    "ukrěpiti" => &[
        VerbDictionaryEntry { lemma: "ukrěpiti", addition: "", transitive: true, imperfective: false },
    ],
    "ukrěpjati" => &[
        VerbDictionaryEntry { lemma: "ukrěpjati", addition: "", transitive: true, imperfective: true },
    ],
    "ukųsiti" => &[
        VerbDictionaryEntry { lemma: "ukųsiti", addition: "", transitive: true, imperfective: false },
    ],
    "ulagađati" => &[
        VerbDictionaryEntry { lemma: "ulagađati", addition: "", transitive: true, imperfective: true },
    ],
    "ulagoditi" => &[
        VerbDictionaryEntry { lemma: "ulagoditi", addition: "", transitive: true, imperfective: false },
    ],
    "uletěti" => &[
        VerbDictionaryEntry { lemma: "uletěti", addition: "(uleti)", transitive: true, imperfective: false },
    ],
    "ulučati" => &[
        VerbDictionaryEntry { lemma: "ulučati", addition: "", transitive: true, imperfective: true },
    ],
    "ulučiti" => &[
        VerbDictionaryEntry { lemma: "ulučiti", addition: "", transitive: true, imperfective: false },
    ],
    "ulučšati" => &[
        VerbDictionaryEntry { lemma: "ulučšati", addition: "", transitive: true, imperfective: true },
    ],
    "ulučšiti" => &[
        VerbDictionaryEntry { lemma: "ulučšiti", addition: "", transitive: true, imperfective: false },
    ],
    "ulėgšati" => &[
        VerbDictionaryEntry { lemma: "ulėgšati", addition: "", transitive: true, imperfective: true },
    ],
    "ulėgšiti" => &[
        VerbDictionaryEntry { lemma: "ulėgšiti", addition: "", transitive: true, imperfective: false },
    ],
    "ulěpšati" => &[
        VerbDictionaryEntry { lemma: "ulěpšati", addition: "", transitive: true, imperfective: true },
    ],
    "ulěpšiti" => &[
        VerbDictionaryEntry { lemma: "ulěpšiti", addition: "", transitive: true, imperfective: false },
    ],
    "ulětati" => &[
        VerbDictionaryEntry { lemma: "ulětati", addition: "", transitive: true, imperfective: true },
    ],
    "umarjati" => &[
        VerbDictionaryEntry { lemma: "umarjati", addition: "", transitive: true, imperfective: true },
    ],
    "umirati" => &[
        VerbDictionaryEntry { lemma: "umirati", addition: "", transitive: true, imperfective: true },
    ],
    "umoliti" => &[
        VerbDictionaryEntry { lemma: "umoliti", addition: "", transitive: true, imperfective: false },
    ],
    "umoljati" => &[
        VerbDictionaryEntry { lemma: "umoljati", addition: "", transitive: true, imperfective: true },
    ],
    "umoriti" => &[
        VerbDictionaryEntry { lemma: "umoriti", addition: "", transitive: true, imperfective: false },
    ],
    "umožniti" => &[
        VerbDictionaryEntry { lemma: "umožniti", addition: "", transitive: true, imperfective: false },
    ],
    "umožnjati" => &[
        VerbDictionaryEntry { lemma: "umožnjati", addition: "", transitive: true, imperfective: true },
    ],
    "umrěti" => &[
        VerbDictionaryEntry { lemma: "umrěti", addition: "(umre)", transitive: true, imperfective: false },
    ],
    "umyti" => &[
        VerbDictionaryEntry { lemma: "umyti", addition: "", transitive: true, imperfective: false },
    ],
    "umyvati" => &[
        VerbDictionaryEntry { lemma: "umyvati", addition: "", transitive: true, imperfective: true },
    ],
    "umėnšati" => &[
        VerbDictionaryEntry { lemma: "umėnšati", addition: "", transitive: true, imperfective: true },
    ],
    "umėnšiti" => &[
        VerbDictionaryEntry { lemma: "umėnšiti", addition: "", transitive: true, imperfective: false },
    ],
    "uměriti" => &[
        VerbDictionaryEntry { lemma: "uměriti", addition: "", transitive: true, imperfective: false },
    ],
    "uměrjati" => &[
        VerbDictionaryEntry { lemma: "uměrjati", addition: "", transitive: true, imperfective: true },
    ],
    "uměstiti" => &[
        VerbDictionaryEntry { lemma: "uměstiti", addition: "", transitive: true, imperfective: false },
    ],
    "uměti" => &[
        VerbDictionaryEntry { lemma: "uměti", addition: "", transitive: false, imperfective: true },
    ],
    "uměšćati" => &[
        VerbDictionaryEntry { lemma: "uměšćati", addition: "", transitive: true, imperfective: true },
    ],
    "umŕti" => &[
        VerbDictionaryEntry { lemma: "umŕti", addition: "(umre)", transitive: true, imperfective: false },
    ],
    "umŕtviti" => &[
        VerbDictionaryEntry { lemma: "umŕtviti", addition: "", transitive: true, imperfective: false },
    ],
    "umŕtvjati" => &[
        VerbDictionaryEntry { lemma: "umŕtvjati", addition: "", transitive: true, imperfective: true },
    ],
    "unarodniti" => &[
        VerbDictionaryEntry { lemma: "unarodniti", addition: "", transitive: true, imperfective: false },
    ],
    "unarodnjati" => &[
        VerbDictionaryEntry { lemma: "unarodnjati", addition: "", transitive: true, imperfective: true },
    ],
    "unemožniti" => &[
        VerbDictionaryEntry { lemma: "unemožniti", addition: "", transitive: true, imperfective: false },
    ],
    "unemožnjati" => &[
        VerbDictionaryEntry { lemma: "unemožnjati", addition: "", transitive: true, imperfective: true },
    ],
    "uneviniti" => &[
        VerbDictionaryEntry { lemma: "uneviniti", addition: "", transitive: true, imperfective: false },
    ],
    "unevinjati" => &[
        VerbDictionaryEntry { lemma: "unevinjati", addition: "", transitive: true, imperfective: true },
    ],
    "uniziti" => &[
        VerbDictionaryEntry { lemma: "uniziti", addition: "", transitive: true, imperfective: false },
    ],
    "uniščiti" => &[
        VerbDictionaryEntry { lemma: "uniščiti", addition: "", transitive: true, imperfective: false },
    ],
    "unižati" => &[
        VerbDictionaryEntry { lemma: "unižati", addition: "", transitive: true, imperfective: true },
    ],
    "upakovati" => &[
        VerbDictionaryEntry { lemma: "upakovati", addition: "", transitive: true, imperfective: false },
    ],
    "upakovyvati" => &[
        VerbDictionaryEntry { lemma: "upakovyvati", addition: "", transitive: true, imperfective: true },
    ],
    "upasti" => &[
        VerbDictionaryEntry { lemma: "upasti", addition: "(upade)", transitive: true, imperfective: false },
    ],
    "upekti" => &[
        VerbDictionaryEntry { lemma: "upekti", addition: "", transitive: true, imperfective: false },
    ],
    "uperiti" => &[
        VerbDictionaryEntry { lemma: "uperiti", addition: "", transitive: true, imperfective: false },
    ],
    "upodabnjati" => &[
        VerbDictionaryEntry { lemma: "upodabnjati", addition: "", transitive: true, imperfective: true },
    ],
    "upodobniti" => &[
        VerbDictionaryEntry { lemma: "upodobniti", addition: "", transitive: true, imperfective: false },
    ],
    "upokarnjati" => &[
        VerbDictionaryEntry { lemma: "upokarnjati", addition: "", transitive: true, imperfective: true },
    ],
    "upokorniti" => &[
        VerbDictionaryEntry { lemma: "upokorniti", addition: "", transitive: true, imperfective: false },
    ],
    "upotrěbiti" => &[
        VerbDictionaryEntry { lemma: "upotrěbiti", addition: "", transitive: true, imperfective: false },
    ],
    "upotrěbjati" => &[
        VerbDictionaryEntry { lemma: "upotrěbjati", addition: "", transitive: true, imperfective: true },
    ],
    "upraviti" => &[
        VerbDictionaryEntry { lemma: "upraviti", addition: "", transitive: true, imperfective: false },
    ],
    "upravjati" => &[
        VerbDictionaryEntry { lemma: "upravjati", addition: "", transitive: true, imperfective: true },
    ],
    "uprašćati" => &[
        VerbDictionaryEntry { lemma: "uprašćati", addition: "", transitive: true, imperfective: true },
    ],
    "uprostiti" => &[
        VerbDictionaryEntry { lemma: "uprostiti", addition: "", transitive: true, imperfective: false },
    ],
    "upȯlnomoćevati" => &[
        VerbDictionaryEntry { lemma: "upȯlnomoćevati", addition: "", transitive: true, imperfective: true },
    ],
    "upȯlnomoćiti" => &[
        VerbDictionaryEntry { lemma: "upȯlnomoćiti", addition: "", transitive: true, imperfective: false },
    ],
    "uraziti" => &[
        VerbDictionaryEntry { lemma: "uraziti", addition: "", transitive: true, imperfective: false },
    ],
    "uražati" => &[
        VerbDictionaryEntry { lemma: "uražati", addition: "", transitive: true, imperfective: true },
    ],
    "uroditi" => &[
        VerbDictionaryEntry { lemma: "uroditi", addition: "", transitive: true, imperfective: false },
    ],
    "urvati" => &[
        VerbDictionaryEntry { lemma: "urvati", addition: "(urve)", transitive: true, imperfective: false },
    ],
    "uryvati" => &[
        VerbDictionaryEntry { lemma: "uryvati", addition: "", transitive: true, imperfective: true },
    ],
    "uråvniti" => &[
        VerbDictionaryEntry { lemma: "uråvniti", addition: "", transitive: true, imperfective: false },
    ],
    "uråvnjati" => &[
        VerbDictionaryEntry { lemma: "uråvnjati", addition: "", transitive: true, imperfective: true },
    ],
    "uręditi" => &[
        VerbDictionaryEntry { lemma: "uręditi", addition: "", transitive: true, imperfective: false },
    ],
    "uręđati" => &[
        VerbDictionaryEntry { lemma: "uręđati", addition: "", transitive: true, imperfective: true },
    ],
    "urěkati" => &[
        VerbDictionaryEntry { lemma: "urěkati", addition: "", transitive: true, imperfective: true },
    ],
    "urěkti" => &[
        VerbDictionaryEntry { lemma: "urěkti", addition: "", transitive: true, imperfective: false },
    ],
    "urězyvati" => &[
        VerbDictionaryEntry { lemma: "urězyvati", addition: "", transitive: true, imperfective: true },
    ],
    "usiliti" => &[
        VerbDictionaryEntry { lemma: "usiliti", addition: "", transitive: true, imperfective: false },
    ],
    "usiljati" => &[
        VerbDictionaryEntry { lemma: "usiljati", addition: "", transitive: true, imperfective: true },
    ],
    "usilovati" => &[
        VerbDictionaryEntry { lemma: "usilovati", addition: "", transitive: true, imperfective: true },
    ],
    "uskoriti" => &[
        VerbDictionaryEntry { lemma: "uskoriti", addition: "", transitive: true, imperfective: false },
    ],
    "uskorjati" => &[
        VerbDictionaryEntry { lemma: "uskorjati", addition: "", transitive: true, imperfective: true },
    ],
    "usložniti" => &[
        VerbDictionaryEntry { lemma: "usložniti", addition: "", transitive: true, imperfective: false },
    ],
    "usložnjati" => &[
        VerbDictionaryEntry { lemma: "usložnjati", addition: "", transitive: true, imperfective: true },
    ],
    "uslyšati" => &[
        VerbDictionaryEntry { lemma: "uslyšati", addition: "(uslyši)", transitive: true, imperfective: false },
    ],
    "usmŕtiti" => &[
        VerbDictionaryEntry { lemma: "usmŕtiti", addition: "", transitive: true, imperfective: false },
    ],
    "usmŕćati" => &[
        VerbDictionaryEntry { lemma: "usmŕćati", addition: "", transitive: true, imperfective: true },
    ],
    "usnųti" => &[
        VerbDictionaryEntry { lemma: "usnųti", addition: "(usne)", transitive: true, imperfective: false },
    ],
    "uspokajati" => &[
        VerbDictionaryEntry { lemma: "uspokajati", addition: "", transitive: true, imperfective: true },
    ],
    "uspokojiti" => &[
        VerbDictionaryEntry { lemma: "uspokojiti", addition: "", transitive: true, imperfective: false },
    ],
    "usposabjati" => &[
        VerbDictionaryEntry { lemma: "usposabjati", addition: "", transitive: true, imperfective: true },
    ],
    "usposobiti" => &[
        VerbDictionaryEntry { lemma: "usposobiti", addition: "", transitive: true, imperfective: false },
    ],
    "uspravědliviti" => &[
        VerbDictionaryEntry { lemma: "uspravědliviti", addition: "", transitive: true, imperfective: false },
    ],
    "uspravědlivjati" => &[
        VerbDictionaryEntry { lemma: "uspravědlivjati", addition: "", transitive: true, imperfective: true },
    ],
    "uspěti" => &[
        VerbDictionaryEntry { lemma: "uspěti", addition: "", transitive: true, imperfective: false },
    ],
    "uspěvati" => &[
        VerbDictionaryEntry { lemma: "uspěvati", addition: "", transitive: true, imperfective: true },
    ],
    "ustaliti" => &[
        VerbDictionaryEntry { lemma: "ustaliti", addition: "", transitive: true, imperfective: false },
    ],
    "ustaljati" => &[
        VerbDictionaryEntry { lemma: "ustaljati", addition: "", transitive: true, imperfective: true },
    ],
    "ustanavjati" => &[
        VerbDictionaryEntry { lemma: "ustanavjati", addition: "", transitive: true, imperfective: true },
    ],
    "ustanoviti" => &[
        VerbDictionaryEntry { lemma: "ustanoviti", addition: "", transitive: true, imperfective: false },
    ],
    "ustati" => &[
        VerbDictionaryEntry { lemma: "ustati", addition: "(ustane)", transitive: true, imperfective: false },
    ],
    "ustavati" => &[
        VerbDictionaryEntry { lemma: "ustavati", addition: "", transitive: true, imperfective: true },
    ],
    "ustaviti" => &[
        VerbDictionaryEntry { lemma: "ustaviti", addition: "", transitive: true, imperfective: false },
    ],
    "ustavjati" => &[
        VerbDictionaryEntry { lemma: "ustavjati", addition: "", transitive: true, imperfective: true },
    ],
    "ustrajati" => &[
        VerbDictionaryEntry { lemma: "ustrajati", addition: "", transitive: true, imperfective: true },
    ],
    "ustrojiti" => &[
        VerbDictionaryEntry { lemma: "ustrojiti", addition: "", transitive: true, imperfective: false },
    ],
    "ustųpati" => &[
        VerbDictionaryEntry { lemma: "ustųpati", addition: "(+3)", transitive: true, imperfective: true },
        VerbDictionaryEntry { lemma: "ustųpati", addition: "", transitive: true, imperfective: true },
    ],
    "ustųpiti" => &[
        VerbDictionaryEntry { lemma: "ustųpiti", addition: "(+3)", transitive: true, imperfective: false },
        VerbDictionaryEntry { lemma: "ustųpiti", addition: "", transitive: true, imperfective: false },
    ],
    "usyhati" => &[
        VerbDictionaryEntry { lemma: "usyhati", addition: "", transitive: true, imperfective: true },
    ],
    "usěsti" => &[
        VerbDictionaryEntry { lemma: "usěsti", addition: "(usěde)", transitive: true, imperfective: false },
    ],
    "usȯhnųti" => &[
        VerbDictionaryEntry { lemma: "usȯhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "utekti" => &[
        VerbDictionaryEntry { lemma: "utekti", addition: "", transitive: true, imperfective: false },
    ],
    "utekųćiniti" => &[
        VerbDictionaryEntry { lemma: "utekųćiniti", addition: "", transitive: true, imperfective: false },
    ],
    "utekųćinjati" => &[
        VerbDictionaryEntry { lemma: "utekųćinjati", addition: "", transitive: true, imperfective: true },
    ],
    "utirati" => &[
        VerbDictionaryEntry { lemma: "utirati", addition: "", transitive: true, imperfective: true },
    ],
    "utišati" => &[
        VerbDictionaryEntry { lemma: "utišati", addition: "", transitive: true, imperfective: true },
    ],
    "utišiti" => &[
        VerbDictionaryEntry { lemma: "utišiti", addition: "", transitive: true, imperfective: false },
    ],
    "utonųti" => &[
        VerbDictionaryEntry { lemma: "utonųti", addition: "", transitive: true, imperfective: false },
    ],
    "utopiti" => &[
        VerbDictionaryEntry { lemma: "utopiti", addition: "", transitive: true, imperfective: false },
    ],
    "utratiti" => &[
        VerbDictionaryEntry { lemma: "utratiti", addition: "", transitive: true, imperfective: false },
    ],
    "utraćati" => &[
        VerbDictionaryEntry { lemma: "utraćati", addition: "", transitive: true, imperfective: true },
    ],
    "utrudniti" => &[
        VerbDictionaryEntry { lemma: "utrudniti", addition: "", transitive: true, imperfective: false },
    ],
    "utrudnjati" => &[
        VerbDictionaryEntry { lemma: "utrudnjati", addition: "", transitive: true, imperfective: true },
    ],
    "utrěti" => &[
        VerbDictionaryEntry { lemma: "utrěti", addition: "(utre)", transitive: true, imperfective: false },
    ],
    "utvŕditi" => &[
        VerbDictionaryEntry { lemma: "utvŕditi", addition: "", transitive: true, imperfective: false },
    ],
    "utvŕđati" => &[
        VerbDictionaryEntry { lemma: "utvŕđati", addition: "", transitive: true, imperfective: true },
    ],
    "utęžati" => &[
        VerbDictionaryEntry { lemma: "utęžati", addition: "", transitive: true, imperfective: true },
    ],
    "utęžiti" => &[
        VerbDictionaryEntry { lemma: "utęžiti", addition: "", transitive: true, imperfective: false },
    ],
    "utěkati" => &[
        VerbDictionaryEntry { lemma: "utěkati", addition: "", transitive: true, imperfective: true },
    ],
    "utělesniti" => &[
        VerbDictionaryEntry { lemma: "utělesniti", addition: "", transitive: true, imperfective: false },
    ],
    "utělesnjati" => &[
        VerbDictionaryEntry { lemma: "utělesnjati", addition: "", transitive: true, imperfective: true },
    ],
    "utěšati" => &[
        VerbDictionaryEntry { lemma: "utěšati", addition: "", transitive: true, imperfective: true },
    ],
    "utěšiti" => &[
        VerbDictionaryEntry { lemma: "utěšiti", addition: "", transitive: true, imperfective: false },
    ],
    "utŕpěti" => &[
        VerbDictionaryEntry { lemma: "utŕpěti", addition: "(utŕpi)", transitive: true, imperfective: false },
    ],
    "utŕti" => &[
        VerbDictionaryEntry { lemma: "utŕti", addition: "(utre)", transitive: true, imperfective: false },
    ],
    "utȯčniti" => &[
        VerbDictionaryEntry { lemma: "utȯčniti", addition: "", transitive: true, imperfective: false },
    ],
    "utȯčnjati" => &[
        VerbDictionaryEntry { lemma: "utȯčnjati", addition: "", transitive: true, imperfective: true },
    ],
    "uvadnjati" => &[
        VerbDictionaryEntry { lemma: "uvadnjati", addition: "", transitive: true, imperfective: true },
    ],
    "uvaljnjati" => &[
        VerbDictionaryEntry { lemma: "uvaljnjati", addition: "", transitive: true, imperfective: true },
    ],
    "uvažati" => &[
        VerbDictionaryEntry { lemma: "uvažati", addition: "", transitive: true, imperfective: true },
    ],
    "uvažiti" => &[
        VerbDictionaryEntry { lemma: "uvažiti", addition: "", transitive: true, imperfective: false },
    ],
    "uveličati" => &[
        VerbDictionaryEntry { lemma: "uveličati", addition: "", transitive: true, imperfective: true },
    ],
    "uveličiti" => &[
        VerbDictionaryEntry { lemma: "uveličiti", addition: "", transitive: true, imperfective: false },
    ],
    "uviděti" => &[
        VerbDictionaryEntry { lemma: "uviděti", addition: "(uvidi)", transitive: true, imperfective: false },
    ],
    "uvodniti" => &[
        VerbDictionaryEntry { lemma: "uvodniti", addition: "", transitive: true, imperfective: false },
    ],
    "uvoljniti" => &[
        VerbDictionaryEntry { lemma: "uvoljniti", addition: "", transitive: true, imperfective: false },
    ],
    "uvędati" => &[
        VerbDictionaryEntry { lemma: "uvędati", addition: "", transitive: true, imperfective: true },
    ],
    "uvędnųti" => &[
        VerbDictionaryEntry { lemma: "uvędnųti", addition: "", transitive: true, imperfective: false },
    ],
    "uvęznųti" => &[
        VerbDictionaryEntry { lemma: "uvęznųti", addition: "", transitive: true, imperfective: false },
    ],
    "uvědamjati" => &[
        VerbDictionaryEntry { lemma: "uvědamjati", addition: "", transitive: true, imperfective: true },
    ],
    "uvědomiti" => &[
        VerbDictionaryEntry { lemma: "uvědomiti", addition: "", transitive: true, imperfective: false },
    ],
    "uvěkověčiti" => &[
        VerbDictionaryEntry { lemma: "uvěkověčiti", addition: "", transitive: true, imperfective: false },
    ],
    "uvěriti" => &[
        VerbDictionaryEntry { lemma: "uvěriti", addition: "", transitive: true, imperfective: false },
    ],
    "uvěrjati" => &[
        VerbDictionaryEntry { lemma: "uvěrjati", addition: "", transitive: true, imperfective: true },
    ],
    "uzdravjati" => &[
        VerbDictionaryEntry { lemma: "uzdravjati", addition: "", transitive: true, imperfective: true },
    ],
    "uzdråviti" => &[
        VerbDictionaryEntry { lemma: "uzdråviti", addition: "", transitive: true, imperfective: false },
    ],
    "uznati" => &[
        VerbDictionaryEntry { lemma: "uznati", addition: "", transitive: true, imperfective: false },
    ],
    "uznavati" => &[
        VerbDictionaryEntry { lemma: "uznavati", addition: "", transitive: true, imperfective: true },
    ],
    "uzrěti" => &[
        VerbDictionaryEntry { lemma: "uzrěti", addition: "(uzri)", transitive: true, imperfective: false },
    ],
    "uzurpovati" => &[
        VerbDictionaryEntry { lemma: "uzurpovati", addition: "", transitive: true, imperfective: true },
    ],
    "učarovati" => &[
        VerbDictionaryEntry { lemma: "učarovati", addition: "", transitive: true, imperfective: false },
    ],
    "učarovyvati" => &[
        VerbDictionaryEntry { lemma: "učarovyvati", addition: "", transitive: true, imperfective: true },
    ],
    "učiti" => &[
        VerbDictionaryEntry { lemma: "učiti", addition: "", transitive: true, imperfective: true },
    ],
    "učęstvovati" => &[
        VerbDictionaryEntry { lemma: "učęstvovati", addition: "", transitive: true, imperfective: true },
    ],
    "užasati" => &[
        VerbDictionaryEntry { lemma: "užasati", addition: "", transitive: true, imperfective: true },
    ],
    "užasnųti" => &[
        VerbDictionaryEntry { lemma: "užasnųti", addition: "", transitive: true, imperfective: false },
    ],
    "užiti" => &[
        VerbDictionaryEntry { lemma: "užiti", addition: "(užive)", transitive: true, imperfective: false },
    ],
    "uživati" => &[
        VerbDictionaryEntry { lemma: "uživati", addition: "", transitive: true, imperfective: true },
    ],
    "vabiti" => &[
        VerbDictionaryEntry { lemma: "vabiti", addition: "", transitive: true, imperfective: true },
    ],
    "vajati" => &[
        VerbDictionaryEntry { lemma: "vajati", addition: "", transitive: true, imperfective: true },
    ],
    "valiti" => &[
        VerbDictionaryEntry { lemma: "valiti", addition: "", transitive: true, imperfective: true },
    ],
    "valjati" => &[
        VerbDictionaryEntry { lemma: "valjati", addition: "", transitive: true, imperfective: true },
    ],
    "valjcevati" => &[
        VerbDictionaryEntry { lemma: "valjcevati", addition: "", transitive: true, imperfective: true },
    ],
    "valsovati" => &[
        VerbDictionaryEntry { lemma: "valsovati", addition: "", transitive: true, imperfective: true },
    ],
    "variovati" => &[
        VerbDictionaryEntry { lemma: "variovati", addition: "", transitive: true, imperfective: true },
    ],
    "variti" => &[
        VerbDictionaryEntry { lemma: "variti", addition: "", transitive: true, imperfective: true },
    ],
    "važiti" => &[
        VerbDictionaryEntry { lemma: "važiti", addition: "", transitive: true, imperfective: true },
    ],
    "vdyhati" => &[
        VerbDictionaryEntry { lemma: "vdyhati", addition: "", transitive: true, imperfective: true },
    ],
    "vdȯhnųti" => &[
        VerbDictionaryEntry { lemma: "vdȯhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "velěti" => &[
        VerbDictionaryEntry { lemma: "velěti", addition: "(veli)", transitive: true, imperfective: true },
    ],
    "ventilovati" => &[
        VerbDictionaryEntry { lemma: "ventilovati", addition: "", transitive: true, imperfective: true },
    ],
    "verbovati" => &[
        VerbDictionaryEntry { lemma: "verbovati", addition: "", transitive: true, imperfective: true },
    ],
    "verifikovati" => &[
        VerbDictionaryEntry { lemma: "verifikovati", addition: "", transitive: true, imperfective: true },
    ],
    "veseliti" => &[
        VerbDictionaryEntry { lemma: "veseliti", addition: "", transitive: true, imperfective: true },
    ],
    "vesti" => &[
        VerbDictionaryEntry { lemma: "vesti", addition: "(vede)", transitive: true, imperfective: true },
    ],
    "vezti" => &[
        VerbDictionaryEntry { lemma: "vezti", addition: "", transitive: true, imperfective: true },
    ],
    "večerjati" => &[
        VerbDictionaryEntry { lemma: "večerjati", addition: "", transitive: true, imperfective: true },
    ],
    "vhoditi" => &[
        VerbDictionaryEntry { lemma: "vhoditi", addition: "", transitive: true, imperfective: true },
    ],
    "vibrovati" => &[
        VerbDictionaryEntry { lemma: "vibrovati", addition: "", transitive: true, imperfective: true },
    ],
    "viděti" => &[
        VerbDictionaryEntry { lemma: "viděti", addition: "(vidi)", transitive: true, imperfective: true },
    ],
    "visěti" => &[
        VerbDictionaryEntry { lemma: "visěti", addition: "(visi)", transitive: true, imperfective: true },
    ],
    "vitati" => &[
        VerbDictionaryEntry { lemma: "vitati", addition: "", transitive: true, imperfective: true },
    ],
    "viti" => &[
        VerbDictionaryEntry { lemma: "viti", addition: "(vije)", transitive: true, imperfective: true },
    ],
    "vjehati" => &[
        VerbDictionaryEntry { lemma: "vjehati", addition: "(vjede)", transitive: true, imperfective: false },
    ],
    "vježđati" => &[
        VerbDictionaryEntry { lemma: "vježđati", addition: "", transitive: true, imperfective: true },
    ],
    "vkladati" => &[
        VerbDictionaryEntry { lemma: "vkladati", addition: "", transitive: true, imperfective: true },
    ],
    "vključati" => &[
        VerbDictionaryEntry { lemma: "vključati", addition: "", transitive: true, imperfective: true },
    ],
    "vključiti" => &[
        VerbDictionaryEntry { lemma: "vključiti", addition: "", transitive: true, imperfective: false },
    ],
    "vkųsiti" => &[
        VerbDictionaryEntry { lemma: "vkųsiti", addition: "", transitive: true, imperfective: false },
    ],
    "vkųšati" => &[
        VerbDictionaryEntry { lemma: "vkųšati", addition: "", transitive: true, imperfective: true },
    ],
    "vladati" => &[
        VerbDictionaryEntry { lemma: "vladati", addition: "(+5)", transitive: true, imperfective: true },
    ],
    "vlagati" => &[
        VerbDictionaryEntry { lemma: "vlagati", addition: "", transitive: true, imperfective: true },
    ],
    "vlastniti" => &[
        VerbDictionaryEntry { lemma: "vlastniti", addition: "", transitive: true, imperfective: true },
    ],
    "vlivati" => &[
        VerbDictionaryEntry { lemma: "vlivati", addition: "", transitive: true, imperfective: true },
    ],
    "vložiti" => &[
        VerbDictionaryEntry { lemma: "vložiti", addition: "", transitive: true, imperfective: false },
    ],
    "vlåděti" => &[
        VerbDictionaryEntry { lemma: "vlåděti", addition: "(+5)", transitive: true, imperfective: true },
    ],
    "vlåčiti" => &[
        VerbDictionaryEntry { lemma: "vlåčiti", addition: "", transitive: true, imperfective: true },
    ],
    "vlåžiti" => &[
        VerbDictionaryEntry { lemma: "vlåžiti", addition: "", transitive: true, imperfective: false },
    ],
    "vlěkti" => &[
        VerbDictionaryEntry { lemma: "vlěkti", addition: "", transitive: true, imperfective: true },
    ],
    "vlězati" => &[
        VerbDictionaryEntry { lemma: "vlězati", addition: "", transitive: true, imperfective: true },
    ],
    "vměstiti" => &[
        VerbDictionaryEntry { lemma: "vměstiti", addition: "", transitive: true, imperfective: false },
    ],
    "vměšati" => &[
        VerbDictionaryEntry { lemma: "vměšati", addition: "", transitive: true, imperfective: false },
    ],
    "vměšivati" => &[
        VerbDictionaryEntry { lemma: "vměšivati", addition: "", transitive: true, imperfective: true },
    ],
    "vměšćati" => &[
        VerbDictionaryEntry { lemma: "vměšćati", addition: "", transitive: true, imperfective: true },
    ],
    "vnesti" => &[
        VerbDictionaryEntry { lemma: "vnesti", addition: "", transitive: true, imperfective: false },
    ],
    "vnikati" => &[
        VerbDictionaryEntry { lemma: "vnikati", addition: "", transitive: true, imperfective: true },
    ],
    "vniknųti" => &[
        VerbDictionaryEntry { lemma: "vniknųti", addition: "", transitive: true, imperfective: false },
    ],
    "vnositi" => &[
        VerbDictionaryEntry { lemma: "vnositi", addition: "", transitive: true, imperfective: true },
    ],
    "voditi" => &[
        VerbDictionaryEntry { lemma: "voditi", addition: "", transitive: true, imperfective: true },
    ],
    "vojevati" => &[
        VerbDictionaryEntry { lemma: "vojevati", addition: "", transitive: true, imperfective: true },
    ],
    "volěti" => &[
        VerbDictionaryEntry { lemma: "volěti", addition: "(voli)", transitive: true, imperfective: true },
    ],
    "vonjati" => &[
        VerbDictionaryEntry { lemma: "vonjati", addition: "", transitive: true, imperfective: true },
    ],
    "voziti" => &[
        VerbDictionaryEntry { lemma: "voziti", addition: "", transitive: true, imperfective: true },
    ],
    "voščiti" => &[
        VerbDictionaryEntry { lemma: "voščiti", addition: "", transitive: true, imperfective: true },
    ],
    "vpadati" => &[
        VerbDictionaryEntry { lemma: "vpadati", addition: "", transitive: true, imperfective: true },
    ],
    "vpasti" => &[
        VerbDictionaryEntry { lemma: "vpasti", addition: "(vpade)", transitive: true, imperfective: false },
    ],
    "vpihati" => &[
        VerbDictionaryEntry { lemma: "vpihati", addition: "", transitive: true, imperfective: true },
    ],
    "vpisati" => &[
        VerbDictionaryEntry { lemma: "vpisati", addition: "(vpiše)", transitive: true, imperfective: false },
    ],
    "vpisyvati" => &[
        VerbDictionaryEntry { lemma: "vpisyvati", addition: "", transitive: true, imperfective: true },
    ],
    "vpiti" => &[
        VerbDictionaryEntry { lemma: "vpiti", addition: "(vpije)", transitive: true, imperfective: false },
    ],
    "vpivati" => &[
        VerbDictionaryEntry { lemma: "vpivati", addition: "", transitive: true, imperfective: true },
    ],
    "vplesti" => &[
        VerbDictionaryEntry { lemma: "vplesti", addition: "(vplete)", transitive: true, imperfective: false },
    ],
    "vpletati" => &[
        VerbDictionaryEntry { lemma: "vpletati", addition: "", transitive: true, imperfective: true },
    ],
    "vplyvati" => &[
        VerbDictionaryEntry { lemma: "vplyvati", addition: "", transitive: true, imperfective: true },
    ],
    "vplyvti" => &[
        VerbDictionaryEntry { lemma: "vplyvti", addition: "", transitive: true, imperfective: false },
    ],
    "vpręgati" => &[
        VerbDictionaryEntry { lemma: "vpręgati", addition: "", transitive: true, imperfective: true },
    ],
    "vpręgti" => &[
        VerbDictionaryEntry { lemma: "vpręgti", addition: "", transitive: true, imperfective: false },
    ],
    "vraćati" => &[
        VerbDictionaryEntry { lemma: "vraćati", addition: "", transitive: true, imperfective: true },
    ],
    "vråtiti" => &[
        VerbDictionaryEntry { lemma: "vråtiti", addition: "", transitive: true, imperfective: false },
    ],
    "vråžiti" => &[
        VerbDictionaryEntry { lemma: "vråžiti", addition: "", transitive: true, imperfective: true },
    ],
    "vrčati" => &[
        VerbDictionaryEntry { lemma: "vrčati", addition: "(vrči)", transitive: true, imperfective: true },
    ],
    "vrěti" => &[
        VerbDictionaryEntry { lemma: "vrěti", addition: "(vri)", transitive: true, imperfective: true },
    ],
    "vrěščati" => &[
        VerbDictionaryEntry { lemma: "vrěščati", addition: "(vrěšči)", transitive: true, imperfective: true },
    ],
    "vrųbati" => &[
        VerbDictionaryEntry { lemma: "vrųbati", addition: "", transitive: true, imperfective: false },
    ],
    "vrųbyvati" => &[
        VerbDictionaryEntry { lemma: "vrųbyvati", addition: "", transitive: true, imperfective: true },
    ],
    "vrųčati" => &[
        VerbDictionaryEntry { lemma: "vrųčati", addition: "", transitive: true, imperfective: true },
    ],
    "vrųčiti" => &[
        VerbDictionaryEntry { lemma: "vrųčiti", addition: "", transitive: true, imperfective: false },
    ],
    "vsaditi" => &[
        VerbDictionaryEntry { lemma: "vsaditi", addition: "", transitive: true, imperfective: false },
    ],
    "vsađati" => &[
        VerbDictionaryEntry { lemma: "vsađati", addition: "", transitive: true, imperfective: true },
    ],
    "vskočiti" => &[
        VerbDictionaryEntry { lemma: "vskočiti", addition: "", transitive: true, imperfective: false },
    ],
    "vstati" => &[
        VerbDictionaryEntry { lemma: "vstati", addition: "(vstane)", transitive: true, imperfective: false },
    ],
    "vstavati" => &[
        VerbDictionaryEntry { lemma: "vstavati", addition: "", transitive: true, imperfective: true },
    ],
    "vstaviti" => &[
        VerbDictionaryEntry { lemma: "vstaviti", addition: "", transitive: true, imperfective: false },
    ],
    "vstavjati" => &[
        VerbDictionaryEntry { lemma: "vstavjati", addition: "", transitive: true, imperfective: true },
    ],
    "vstųpati" => &[
        VerbDictionaryEntry { lemma: "vstųpati", addition: "", transitive: true, imperfective: true },
    ],
    "vstųpiti" => &[
        VerbDictionaryEntry { lemma: "vstųpiti", addition: "", transitive: true, imperfective: false },
    ],
    "vtiskati" => &[
        VerbDictionaryEntry { lemma: "vtiskati", addition: "", transitive: true, imperfective: true },
    ],
    "vtisknųti" => &[
        VerbDictionaryEntry { lemma: "vtisknųti", addition: "", transitive: true, imperfective: false },
    ],
    "vtrgati" => &[
        VerbDictionaryEntry { lemma: "vtrgati", addition: "", transitive: true, imperfective: true },
    ],
    "vtrgnųti" => &[
        VerbDictionaryEntry { lemma: "vtrgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "vtęgati" => &[
        VerbDictionaryEntry { lemma: "vtęgati", addition: "", transitive: true, imperfective: true },
    ],
    "vtęgnųti" => &[
        VerbDictionaryEntry { lemma: "vtęgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "vulkanizovati" => &[
        VerbDictionaryEntry { lemma: "vulkanizovati", addition: "", transitive: true, imperfective: true },
    ],
    "vvesti" => &[
        VerbDictionaryEntry { lemma: "vvesti", addition: "(vvede)", transitive: true, imperfective: false },
    ],
    "vvezti" => &[
        VerbDictionaryEntry { lemma: "vvezti", addition: "", transitive: true, imperfective: false },
    ],
    "vvoditi" => &[
        VerbDictionaryEntry { lemma: "vvoditi", addition: "", transitive: true, imperfective: true },
    ],
    "vvoziti" => &[
        VerbDictionaryEntry { lemma: "vvoziti", addition: "", transitive: true, imperfective: true },
    ],
    "vvŕgati" => &[
        VerbDictionaryEntry { lemma: "vvŕgati", addition: "", transitive: true, imperfective: true },
    ],
    "vvŕgnųti" => &[
        VerbDictionaryEntry { lemma: "vvŕgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "vyjdti" => &[
        VerbDictionaryEntry { lemma: "vyjdti", addition: "(vyjde; vyšėl)", transitive: true, imperfective: false },
    ],
    "vyjmati" => &[
        VerbDictionaryEntry { lemma: "vyjmati", addition: "", transitive: true, imperfective: true },
    ],
    "vyryti" => &[
        VerbDictionaryEntry { lemma: "vyryti", addition: "", transitive: true, imperfective: false },
    ],
    "vyti" => &[
        VerbDictionaryEntry { lemma: "vyti", addition: "", transitive: true, imperfective: true },
    ],
    "vzajėmodějati" => &[
        VerbDictionaryEntry { lemma: "vzajėmodějati", addition: "(vzajemoděje)", transitive: true, imperfective: true },
    ],
    "vzajėmodějstvovati" => &[
        VerbDictionaryEntry { lemma: "vzajėmodějstvovati", addition: "", transitive: true, imperfective: true },
    ],
    "vzęti" => &[
        VerbDictionaryEntry { lemma: "vzęti", addition: "(vȯzme)", transitive: true, imperfective: false },
    ],
    "včleniti" => &[
        VerbDictionaryEntry { lemma: "včleniti", addition: "", transitive: true, imperfective: false },
    ],
    "včlenjati" => &[
        VerbDictionaryEntry { lemma: "včlenjati", addition: "", transitive: true, imperfective: true },
    ],
    "vędnųti" => &[
        VerbDictionaryEntry { lemma: "vędnųti", addition: "", transitive: true, imperfective: true },
    ],
    "vęzati" => &[
        VerbDictionaryEntry { lemma: "vęzati", addition: "(vęže)", transitive: true, imperfective: true },
    ],
    "vęznųti" => &[
        VerbDictionaryEntry { lemma: "vęznųti", addition: "", transitive: true, imperfective: true },
    ],
    "věděti" => &[
        VerbDictionaryEntry { lemma: "věděti", addition: "(vě)", transitive: true, imperfective: true },
    ],
    "vějati" => &[
        VerbDictionaryEntry { lemma: "vějati", addition: "(věje)", transitive: true, imperfective: true },
    ],
    "věriti" => &[
        VerbDictionaryEntry { lemma: "věriti", addition: "", transitive: true, imperfective: true },
    ],
    "větriti" => &[
        VerbDictionaryEntry { lemma: "větriti", addition: "", transitive: true, imperfective: true },
    ],
    "věšati" => &[
        VerbDictionaryEntry { lemma: "věšati", addition: "", transitive: true, imperfective: true },
    ],
    "vŕgati" => &[
        VerbDictionaryEntry { lemma: "vŕgati", addition: "", transitive: true, imperfective: true },
    ],
    "vŕgnųti" => &[
        VerbDictionaryEntry { lemma: "vŕgnųti", addition: "", transitive: true, imperfective: false },
    ],
    "vŕnųti" => &[
        VerbDictionaryEntry { lemma: "vŕnųti", addition: "", transitive: true, imperfective: false },
    ],
    "vŕtěti" => &[
        VerbDictionaryEntry { lemma: "vŕtěti", addition: "(vŕti)", transitive: true, imperfective: true },
    ],
    "vȯjdti" => &[
        VerbDictionaryEntry { lemma: "vȯjdti", addition: "(vȯjde; všėl)", transitive: true, imperfective: false },
    ],
    "vȯlgnųti" => &[
        VerbDictionaryEntry { lemma: "vȯlgnųti", addition: "", transitive: true, imperfective: true },
    ],
    "vȯorųžiti" => &[
        VerbDictionaryEntry { lemma: "vȯorųžiti", addition: "", transitive: true, imperfective: false },
    ],
    "vȯphati" => &[
        VerbDictionaryEntry { lemma: "vȯphati", addition: "", transitive: true, imperfective: false },
    ],
    "vȯplȯtiti" => &[
        VerbDictionaryEntry { lemma: "vȯplȯtiti", addition: "", transitive: true, imperfective: false },
    ],
    "vȯplȯćati" => &[
        VerbDictionaryEntry { lemma: "vȯplȯćati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzbogatiti" => &[
        VerbDictionaryEntry { lemma: "vȯzbogatiti", addition: "", transitive: true, imperfective: false },
    ],
    "vȯzbogaćati" => &[
        VerbDictionaryEntry { lemma: "vȯzbogaćati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzbranjati" => &[
        VerbDictionaryEntry { lemma: "vȯzbranjati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzbråniti" => &[
        VerbDictionaryEntry { lemma: "vȯzbråniti", addition: "", transitive: true, imperfective: false },
    ],
    "vȯzbuditi" => &[
        VerbDictionaryEntry { lemma: "vȯzbuditi", addition: "", transitive: true, imperfective: false },
    ],
    "vȯzbuđati" => &[
        VerbDictionaryEntry { lemma: "vȯzbuđati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzdvignųti" => &[
        VerbDictionaryEntry { lemma: "vȯzdvignųti", addition: "", transitive: true, imperfective: false },
    ],
    "vȯzdyhati" => &[
        VerbDictionaryEntry { lemma: "vȯzdyhati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzdȯhnųti" => &[
        VerbDictionaryEntry { lemma: "vȯzdȯhnųti", addition: "", transitive: true, imperfective: false },
    ],
    "vȯzhoditi" => &[
        VerbDictionaryEntry { lemma: "vȯzhoditi", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzimati" => &[
        VerbDictionaryEntry { lemma: "vȯzimati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzjęti" => &[
        VerbDictionaryEntry { lemma: "vȯzjęti", addition: "(vȯzȯjme)", transitive: true, imperfective: false },
    ],
    "vȯzklicati" => &[
        VerbDictionaryEntry { lemma: "vȯzklicati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzkliknųti" => &[
        VerbDictionaryEntry { lemma: "vȯzkliknųti", addition: "", transitive: true, imperfective: false },
    ],
    "vȯzkresati" => &[
        VerbDictionaryEntry { lemma: "vȯzkresati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzkresiti" => &[
        VerbDictionaryEntry { lemma: "vȯzkresiti", addition: "", transitive: true, imperfective: false },
    ],
    "vȯzkresnųti" => &[
        VerbDictionaryEntry { lemma: "vȯzkresnųti", addition: "", transitive: true, imperfective: false },
    ],
    "vȯzkrešati" => &[
        VerbDictionaryEntry { lemma: "vȯzkrešati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzletěti" => &[
        VerbDictionaryEntry { lemma: "vȯzletěti", addition: "(vȯzleti)", transitive: true, imperfective: false },
    ],
    "vȯzlětati" => &[
        VerbDictionaryEntry { lemma: "vȯzlětati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯznesti" => &[
        VerbDictionaryEntry { lemma: "vȯznesti", addition: "", transitive: true, imperfective: false },
    ],
    "vȯznikati" => &[
        VerbDictionaryEntry { lemma: "vȯznikati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzniknųti" => &[
        VerbDictionaryEntry { lemma: "vȯzniknųti", addition: "", transitive: true, imperfective: false },
    ],
    "vȯznositi" => &[
        VerbDictionaryEntry { lemma: "vȯznositi", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzpamętati" => &[
        VerbDictionaryEntry { lemma: "vȯzpamętati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzpitati" => &[
        VerbDictionaryEntry { lemma: "vȯzpitati", addition: "", transitive: true, imperfective: false },
    ],
    "vȯzpityvati" => &[
        VerbDictionaryEntry { lemma: "vȯzpityvati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzpominati" => &[
        VerbDictionaryEntry { lemma: "vȯzpominati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzpomněti" => &[
        VerbDictionaryEntry { lemma: "vȯzpomněti", addition: "(vȯzpomni)", transitive: true, imperfective: false },
    ],
    "vȯzrastati" => &[
        VerbDictionaryEntry { lemma: "vȯzrastati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzroditi" => &[
        VerbDictionaryEntry { lemma: "vȯzroditi", addition: "", transitive: true, imperfective: false },
    ],
    "vȯzråsti" => &[
        VerbDictionaryEntry { lemma: "vȯzråsti", addition: "(vȯzråste)", transitive: true, imperfective: false },
    ],
    "vȯzsiliti" => &[
        VerbDictionaryEntry { lemma: "vȯzsiliti", addition: "", transitive: true, imperfective: false },
    ],
    "vȯzsilovati" => &[
        VerbDictionaryEntry { lemma: "vȯzsilovati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzskakati" => &[
        VerbDictionaryEntry { lemma: "vȯzskakati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzskočiti" => &[
        VerbDictionaryEntry { lemma: "vȯzskočiti", addition: "", transitive: true, imperfective: false },
    ],
    "vȯzstati" => &[
        VerbDictionaryEntry { lemma: "vȯzstati", addition: "(vȯzstane)", transitive: true, imperfective: false },
    ],
    "vȯzstavati" => &[
        VerbDictionaryEntry { lemma: "vȯzstavati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯztręsati" => &[
        VerbDictionaryEntry { lemma: "vȯztręsati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯztręsti" => &[
        VerbDictionaryEntry { lemma: "vȯztręsti", addition: "", transitive: true, imperfective: false },
    ],
    "vȯzveličati" => &[
        VerbDictionaryEntry { lemma: "vȯzveličati", addition: "", transitive: true, imperfective: false },
    ],
    "vȯzveličivati" => &[
        VerbDictionaryEntry { lemma: "vȯzveličivati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzvesti" => &[
        VerbDictionaryEntry { lemma: "vȯzvesti", addition: "(vȯzvede)", transitive: true, imperfective: false },
    ],
    "vȯzvoditi" => &[
        VerbDictionaryEntry { lemma: "vȯzvoditi", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzvraćati" => &[
        VerbDictionaryEntry { lemma: "vȯzvraćati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzvråtiti" => &[
        VerbDictionaryEntry { lemma: "vȯzvråtiti", addition: "", transitive: true, imperfective: false },
    ],
    "vȯzvysiti" => &[
        VerbDictionaryEntry { lemma: "vȯzvysiti", addition: "", transitive: true, imperfective: false },
    ],
    "vȯzvyšati" => &[
        VerbDictionaryEntry { lemma: "vȯzvyšati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzvŕšati" => &[
        VerbDictionaryEntry { lemma: "vȯzvŕšati", addition: "", transitive: true, imperfective: true },
    ],
    "vȯzvŕšiti" => &[
        VerbDictionaryEntry { lemma: "vȯzvŕšiti", addition: "", transitive: true, imperfective: false },
    ],
    "vȯzȯjdti" => &[
        VerbDictionaryEntry { lemma: "vȯzȯjdti", addition: "(vȯzȯjde; vȯzšėl)", transitive: true, imperfective: false },
    ],
    "zabarikadovati" => &[
        VerbDictionaryEntry { lemma: "zabarikadovati", addition: "", transitive: true, imperfective: false },
    ],
    "zabaviti" => &[
        VerbDictionaryEntry { lemma: "zabaviti", addition: "", transitive: true, imperfective: false },
    ],
    "zabavjati" => &[
        VerbDictionaryEntry { lemma: "zabavjati", addition: "", transitive: true, imperfective: true },
    ],
    "zabezpamętiti" => &[
        VerbDictionaryEntry { lemma: "zabezpamętiti", addition: "", transitive: true, imperfective: false },
    ],
    "zabezpamęćati" => &[
        VerbDictionaryEntry { lemma: "zabezpamęćati", addition: "", transitive: true, imperfective: true },
    ],
    "zabezpečati" => &[
        VerbDictionaryEntry { lemma: "zabezpečati", addition: "", transitive: true, imperfective: true },
    ],
    "zabezpečiti" => &[
        VerbDictionaryEntry { lemma: "zabezpečiti", addition: "", transitive: true, imperfective: false },
    ],
    "zabirati" => &[
        VerbDictionaryEntry { lemma: "zabirati", addition: "", transitive: true, imperfective: true },
    ],
    "zablokovati" => &[
        VerbDictionaryEntry { lemma: "zablokovati", addition: "", transitive: true, imperfective: false },
    ],
    "zablųditi" => &[
        VerbDictionaryEntry { lemma: "zablųditi", addition: "", transitive: true, imperfective: false },
    ],
    "zablųđati" => &[
        VerbDictionaryEntry { lemma: "zablųđati", addition: "", transitive: true, imperfective: true },
    ],
    "zabolěti" => &[
        VerbDictionaryEntry { lemma: "zabolěti", addition: "(zabolěje)", transitive: true, imperfective: false },
        VerbDictionaryEntry { lemma: "zabolěti", addition: "(zaboli)", transitive: true, imperfective: false },
    ],
    "zabolěvati" => &[
        VerbDictionaryEntry { lemma: "zabolěvati", addition: "", transitive: true, imperfective: true },
    ],
    "zabranjati" => &[
        VerbDictionaryEntry { lemma: "zabranjati", addition: "", transitive: true, imperfective: true },
    ],
    "zabrati" => &[
        VerbDictionaryEntry { lemma: "zabrati", addition: "(zabere)", transitive: true, imperfective: false },
    ],
    "zabråniti" => &[
        VerbDictionaryEntry { lemma: "zabråniti", addition: "", transitive: true, imperfective: false },
    ],
    "zabyti" => &[
        VerbDictionaryEntry { lemma: "zabyti", addition: "(zabųde)", transitive: true, imperfective: false },
    ],
    "zabyvati" => &[
        VerbDictionaryEntry { lemma: "zabyvati", addition: "", transitive: true, imperfective: true },
    ],
    "zadati" => &[
        VerbDictionaryEntry { lemma: "zadati", addition: "(zada)", transitive: true, imperfective: false },
    ],
    "zadavati" => &[
        VerbDictionaryEntry { lemma: "zadavati", addition: "", transitive: true, imperfective: true },
    ],
    "zadovaljati" => &[
        VerbDictionaryEntry { lemma: "zadovaljati", addition: "", transitive: true, imperfective: true },
    ],
    "zadovoliti" => &[
        VerbDictionaryEntry { lemma: "zadovoliti", addition: "", transitive: true, imperfective: false },
    ],
    "zadrěmati" => &[
        VerbDictionaryEntry { lemma: "zadrěmati", addition: "", transitive: true, imperfective: false },
    ],
    "zadržati" => &[
        VerbDictionaryEntry { lemma: "zadržati", addition: "(zadŕži)", transitive: true, imperfective: false },
    ],
    "zadŕžati" => &[
        VerbDictionaryEntry { lemma: "zadŕžati", addition: "(zadŕži)", transitive: true, imperfective: false },
    ],
    "zadŕživati" => &[
        VerbDictionaryEntry { lemma: "zadŕživati", addition: "", transitive: true, imperfective: true },
    ],
    "zagladiti" => &[
        VerbDictionaryEntry { lemma: "zagladiti", addition: "", transitive: true, imperfective: false },
    ],
    "zaglađati" => &[
        VerbDictionaryEntry { lemma: "zaglađati", addition: "", transitive: true, imperfective: true },
    ],
    "zagorěti" => &[
        VerbDictionaryEntry { lemma: "zagorěti", addition: "", transitive: true, imperfective: false },
    ],
    "zagospodariti" => &[
        VerbDictionaryEntry { lemma: "zagospodariti", addition: "", transitive: true, imperfective: false },
    ],
    "zagroziti" => &[
        VerbDictionaryEntry { lemma: "zagroziti", addition: "", transitive: true, imperfective: false },
    ],
    "zahoditi" => &[
        VerbDictionaryEntry { lemma: "zahoditi", addition: "", transitive: true, imperfective: true },
    ],
    "zahvatiti" => &[
        VerbDictionaryEntry { lemma: "zahvatiti", addition: "", transitive: true, imperfective: false },
    ],
    "zahvaćati" => &[
        VerbDictionaryEntry { lemma: "zahvaćati", addition: "", transitive: true, imperfective: true },
    ],
    "zahvorěti" => &[
        VerbDictionaryEntry { lemma: "zahvorěti", addition: "", transitive: true, imperfective: false },
    ],
    "zahvorěvati" => &[
        VerbDictionaryEntry { lemma: "zahvorěvati", addition: "", transitive: true, imperfective: true },
    ],
    "zainteresovati" => &[
        VerbDictionaryEntry { lemma: "zainteresovati", addition: "", transitive: true, imperfective: false },
    ],
    "zajdti" => &[
        VerbDictionaryEntry { lemma: "zajdti", addition: "(zajde; zašėl)", transitive: true, imperfective: false },
    ],
    "zajmati" => &[
        VerbDictionaryEntry { lemma: "zajmati", addition: "", transitive: true, imperfective: true },
    ],
    "zajęti" => &[
        VerbDictionaryEntry { lemma: "zajęti", addition: "(zajme)", transitive: true, imperfective: false },
    ],
    "zakazyvati" => &[
        VerbDictionaryEntry { lemma: "zakazyvati", addition: "", transitive: true, imperfective: true },
    ],
    "zakašljati" => &[
        VerbDictionaryEntry { lemma: "zakašljati", addition: "", transitive: true, imperfective: false },
    ],
    "zakladati" => &[
        VerbDictionaryEntry { lemma: "zakladati", addition: "", transitive: true, imperfective: true },
    ],
    "zaklinati" => &[
        VerbDictionaryEntry { lemma: "zaklinati", addition: "", transitive: true, imperfective: true },
    ],
    "zakliniti" => &[
        VerbDictionaryEntry { lemma: "zakliniti", addition: "", transitive: true, imperfective: false },
    ],
    "zaklinovati" => &[
        VerbDictionaryEntry { lemma: "zaklinovati", addition: "", transitive: true, imperfective: true },
    ],
    "zaključati" => &[
        VerbDictionaryEntry { lemma: "zaključati", addition: "", transitive: true, imperfective: true },
    ],
    "zaključiti" => &[
        VerbDictionaryEntry { lemma: "zaključiti", addition: "", transitive: true, imperfective: false },
    ],
    "zaklåti" => &[
        VerbDictionaryEntry { lemma: "zaklåti", addition: "(zakolje)", transitive: true, imperfective: false },
    ],
    "zaklęti" => &[
        VerbDictionaryEntry { lemma: "zaklęti", addition: "(zaklne)", transitive: true, imperfective: false },
    ],
    "zakončiti" => &[
        VerbDictionaryEntry { lemma: "zakončiti", addition: "", transitive: true, imperfective: false },
    ],
    "zakopati" => &[
        VerbDictionaryEntry { lemma: "zakopati", addition: "", transitive: true, imperfective: false },
    ],
    "zakopyvati" => &[
        VerbDictionaryEntry { lemma: "zakopyvati", addition: "", transitive: true, imperfective: true },
    ],
    "zakričati" => &[
        VerbDictionaryEntry { lemma: "zakričati", addition: "", transitive: true, imperfective: false },
    ],
    "zakryti" => &[
        VerbDictionaryEntry { lemma: "zakryti", addition: "", transitive: true, imperfective: false },
    ],
    "zakryvati" => &[
        VerbDictionaryEntry { lemma: "zakryvati", addition: "", transitive: true, imperfective: true },
    ],
    "zalajati" => &[
        VerbDictionaryEntry { lemma: "zalajati", addition: "", transitive: true, imperfective: false },
    ],
    "založiti" => &[
        VerbDictionaryEntry { lemma: "založiti", addition: "", transitive: true, imperfective: false },
    ],
    "zalězati" => &[
        VerbDictionaryEntry { lemma: "zalězati", addition: "", transitive: true, imperfective: true },
    ],
    "zalězti" => &[
        VerbDictionaryEntry { lemma: "zalězti", addition: "", transitive: true, imperfective: false },
    ],
    "zamesti" => &[
        VerbDictionaryEntry { lemma: "zamesti", addition: "(zamete)", transitive: true, imperfective: false },
    ],
    "zamglati" => &[
        VerbDictionaryEntry { lemma: "zamglati", addition: "", transitive: true, imperfective: false },
    ],
    "zamirati" => &[
        VerbDictionaryEntry { lemma: "zamirati", addition: "", transitive: true, imperfective: true },
    ],
    "zamknųti" => &[
        VerbDictionaryEntry { lemma: "zamknųti", addition: "", transitive: true, imperfective: false },
    ],
    "zamražati" => &[
        VerbDictionaryEntry { lemma: "zamražati", addition: "", transitive: true, imperfective: true },
    ],
    "zamrzati" => &[
        VerbDictionaryEntry { lemma: "zamrzati", addition: "", transitive: true, imperfective: true },
    ],
    "zamrznųti" => &[
        VerbDictionaryEntry { lemma: "zamrznųti", addition: "", transitive: true, imperfective: false },
    ],
    "zamråziti" => &[
        VerbDictionaryEntry { lemma: "zamråziti", addition: "", transitive: true, imperfective: false },
    ],
    "zamrěti" => &[
        VerbDictionaryEntry { lemma: "zamrěti", addition: "(zamre)", transitive: true, imperfective: false },
    ],
    "zamykati" => &[
        VerbDictionaryEntry { lemma: "zamykati", addition: "", transitive: true, imperfective: true },
    ],
    "zamysliti" => &[
        VerbDictionaryEntry { lemma: "zamysliti", addition: "", transitive: true, imperfective: false },
    ],
    "zamysljati" => &[
        VerbDictionaryEntry { lemma: "zamysljati", addition: "", transitive: true, imperfective: true },
    ],
    "zamėdliti" => &[
        VerbDictionaryEntry { lemma: "zamėdliti", addition: "", transitive: true, imperfective: false },
    ],
    "zamėdljati" => &[
        VerbDictionaryEntry { lemma: "zamėdljati", addition: "", transitive: true, imperfective: true },
    ],
    "zaměniti" => &[
        VerbDictionaryEntry { lemma: "zaměniti", addition: "", transitive: true, imperfective: false },
    ],
    "zaměnjati" => &[
        VerbDictionaryEntry { lemma: "zaměnjati", addition: "", transitive: true, imperfective: true },
    ],
    "zaměsiti" => &[
        VerbDictionaryEntry { lemma: "zaměsiti", addition: "", transitive: true, imperfective: false },
    ],
    "zamětati" => &[
        VerbDictionaryEntry { lemma: "zamětati", addition: "", transitive: true, imperfective: true },
    ],
    "zaměšati" => &[
        VerbDictionaryEntry { lemma: "zaměšati", addition: "", transitive: true, imperfective: false },
    ],
    "zaměšivati" => &[
        VerbDictionaryEntry { lemma: "zaměšivati", addition: "", transitive: true, imperfective: true },
    ],
    "zamŕti" => &[
        VerbDictionaryEntry { lemma: "zamŕti", addition: "(zamre)", transitive: true, imperfective: false },
    ],
    "zamȯlknųti" => &[
        VerbDictionaryEntry { lemma: "zamȯlknųti", addition: "", transitive: true, imperfective: false },
    ],
    "zamȯlviti" => &[
        VerbDictionaryEntry { lemma: "zamȯlviti", addition: "", transitive: true, imperfective: false },
    ],
    "zamȯlvjati" => &[
        VerbDictionaryEntry { lemma: "zamȯlvjati", addition: "", transitive: true, imperfective: true },
    ],
    "zanedbati" => &[
        VerbDictionaryEntry { lemma: "zanedbati", addition: "", transitive: true, imperfective: false },
    ],
    "zanedbyvati" => &[
        VerbDictionaryEntry { lemma: "zanedbyvati", addition: "", transitive: true, imperfective: true },
    ],
    "zanepokojiti" => &[
        VerbDictionaryEntry { lemma: "zanepokojiti", addition: "", transitive: true, imperfective: false },
    ],
    "zanuriti" => &[
        VerbDictionaryEntry { lemma: "zanuriti", addition: "", transitive: true, imperfective: false },
    ],
    "zanurjati" => &[
        VerbDictionaryEntry { lemma: "zanurjati", addition: "", transitive: true, imperfective: true },
    ],
    "zapadati" => &[
        VerbDictionaryEntry { lemma: "zapadati", addition: "", transitive: true, imperfective: true },
    ],
    "zapakovati" => &[
        VerbDictionaryEntry { lemma: "zapakovati", addition: "", transitive: true, imperfective: false },
    ],
    "zapakovyvati" => &[
        VerbDictionaryEntry { lemma: "zapakovyvati", addition: "", transitive: true, imperfective: true },
    ],
    "zapaliti" => &[
        VerbDictionaryEntry { lemma: "zapaliti", addition: "", transitive: true, imperfective: false },
    ],
    "zapaljati" => &[
        VerbDictionaryEntry { lemma: "zapaljati", addition: "", transitive: true, imperfective: true },
    ],
    "zapamętati" => &[
        VerbDictionaryEntry { lemma: "zapamętati", addition: "", transitive: true, imperfective: false },
    ],
    "zapamętyvati" => &[
        VerbDictionaryEntry { lemma: "zapamętyvati", addition: "", transitive: true, imperfective: true },
    ],
    "zapasti" => &[
        VerbDictionaryEntry { lemma: "zapasti", addition: "(zapade)", transitive: true, imperfective: false },
    ],
    "zapečatati" => &[
        VerbDictionaryEntry { lemma: "zapečatati", addition: "", transitive: true, imperfective: false },
    ],
    "zapečatyvati" => &[
        VerbDictionaryEntry { lemma: "zapečatyvati", addition: "", transitive: true, imperfective: true },
    ],
    "zapisati" => &[
        VerbDictionaryEntry { lemma: "zapisati", addition: "(zapiše)", transitive: true, imperfective: false },
    ],
    "zapisyvati" => &[
        VerbDictionaryEntry { lemma: "zapisyvati", addition: "", transitive: true, imperfective: true },
    ],
    "zaplakati" => &[
        VerbDictionaryEntry { lemma: "zaplakati", addition: "", transitive: true, imperfective: false },
    ],
    "zaplanovati" => &[
        VerbDictionaryEntry { lemma: "zaplanovati", addition: "", transitive: true, imperfective: false },
    ],
    "zaplatiti" => &[
        VerbDictionaryEntry { lemma: "zaplatiti", addition: "", transitive: true, imperfective: false },
    ],
    "zaplěniti" => &[
        VerbDictionaryEntry { lemma: "zaplěniti", addition: "", transitive: true, imperfective: false },
    ],
    "zaplěnjati" => &[
        VerbDictionaryEntry { lemma: "zaplěnjati", addition: "", transitive: true, imperfective: true },
    ],
    "zapoznati" => &[
        VerbDictionaryEntry { lemma: "zapoznati", addition: "", transitive: true, imperfective: false },
    ],
    "zapoznavati" => &[
        VerbDictionaryEntry { lemma: "zapoznavati", addition: "", transitive: true, imperfective: true },
    ],
    "započinati" => &[
        VerbDictionaryEntry { lemma: "započinati", addition: "", transitive: true, imperfective: true },
    ],
    "započęti" => &[
        VerbDictionaryEntry { lemma: "započęti", addition: "(započne)", transitive: true, imperfective: false },
    ],
    "zapraviti" => &[
        VerbDictionaryEntry { lemma: "zapraviti", addition: "", transitive: true, imperfective: false },
    ],
    "zapravjati" => &[
        VerbDictionaryEntry { lemma: "zapravjati", addition: "", transitive: true, imperfective: true },
    ],
    "zaprojektovati" => &[
        VerbDictionaryEntry { lemma: "zaprojektovati", addition: "", transitive: true, imperfective: false },
    ],
    "zapråšiti" => &[
        VerbDictionaryEntry { lemma: "zapråšiti", addition: "", transitive: true, imperfective: false },
    ],
    "zapręgati" => &[
        VerbDictionaryEntry { lemma: "zapręgati", addition: "", transitive: true, imperfective: true },
    ],
    "zapręgti" => &[
        VerbDictionaryEntry { lemma: "zapręgti", addition: "", transitive: true, imperfective: false },
    ],
    "zaprěčiti" => &[
        VerbDictionaryEntry { lemma: "zaprěčiti", addition: "", transitive: true, imperfective: false },
    ],
    "zapustiti" => &[
        VerbDictionaryEntry { lemma: "zapustiti", addition: "", transitive: true, imperfective: false },
    ],
    "zapušćati" => &[
        VerbDictionaryEntry { lemma: "zapušćati", addition: "", transitive: true, imperfective: true },
    ],
    "zapytati" => &[
        VerbDictionaryEntry { lemma: "zapytati", addition: "", transitive: true, imperfective: false },
    ],
    "zapytyvati" => &[
        VerbDictionaryEntry { lemma: "zapytyvati", addition: "", transitive: true, imperfective: true },
    ],
    "zapȯlniti" => &[
        VerbDictionaryEntry { lemma: "zapȯlniti", addition: "", transitive: true, imperfective: false },
    ],
    "zapȯlnjati" => &[
        VerbDictionaryEntry { lemma: "zapȯlnjati", addition: "", transitive: true, imperfective: true },
    ],
    "zarastati" => &[
        VerbDictionaryEntry { lemma: "zarastati", addition: "", transitive: true, imperfective: true },
    ],
    "zaraziti" => &[
        VerbDictionaryEntry { lemma: "zaraziti", addition: "", transitive: true, imperfective: false },
    ],
    "zaražati" => &[
        VerbDictionaryEntry { lemma: "zaražati", addition: "", transitive: true, imperfective: true },
    ],
    "zaregistrovati" => &[
        VerbDictionaryEntry { lemma: "zaregistrovati", addition: "", transitive: true, imperfective: false },
    ],
    "zarevti" => &[
        VerbDictionaryEntry { lemma: "zarevti", addition: "", transitive: true, imperfective: false },
    ],
    "zarezervovati" => &[
        VerbDictionaryEntry { lemma: "zarezervovati", addition: "", transitive: true, imperfective: false },
    ],
    "zarydati" => &[
        VerbDictionaryEntry { lemma: "zarydati", addition: "", transitive: true, imperfective: false },
    ],
    "zaråbotati" => &[
        VerbDictionaryEntry { lemma: "zaråbotati", addition: "", transitive: true, imperfective: false },
    ],
    "zaråbotyvati" => &[
        VerbDictionaryEntry { lemma: "zaråbotyvati", addition: "", transitive: true, imperfective: true },
    ],
    "zaråsti" => &[
        VerbDictionaryEntry { lemma: "zaråsti", addition: "(zaråste)", transitive: true, imperfective: false },
    ],
    "zarųčati" => &[
        VerbDictionaryEntry { lemma: "zarųčati", addition: "", transitive: true, imperfective: true },
    ],
    "zarųčiti" => &[
        VerbDictionaryEntry { lemma: "zarųčiti", addition: "", transitive: true, imperfective: false },
    ],
    "zasaditi" => &[
        VerbDictionaryEntry { lemma: "zasaditi", addition: "", transitive: true, imperfective: false },
    ],
    "zasađati" => &[
        VerbDictionaryEntry { lemma: "zasađati", addition: "", transitive: true, imperfective: true },
    ],
    "zaslanjati" => &[
        VerbDictionaryEntry { lemma: "zaslanjati", addition: "", transitive: true, imperfective: true },
    ],
    "zasloniti" => &[
        VerbDictionaryEntry { lemma: "zasloniti", addition: "", transitive: true, imperfective: false },
    ],
    "zaslužiti" => &[
        VerbDictionaryEntry { lemma: "zaslužiti", addition: "", transitive: true, imperfective: false },
    ],
    "zasluživati" => &[
        VerbDictionaryEntry { lemma: "zasluživati", addition: "", transitive: true, imperfective: true },
    ],
    "zasnųti" => &[
        VerbDictionaryEntry { lemma: "zasnųti", addition: "", transitive: true, imperfective: false },
    ],
    "zaspati" => &[
        VerbDictionaryEntry { lemma: "zaspati", addition: "(zaspi)", transitive: true, imperfective: false },
    ],
    "zasramjati" => &[
        VerbDictionaryEntry { lemma: "zasramjati", addition: "", transitive: true, imperfective: true },
    ],
    "zasrati" => &[
        VerbDictionaryEntry { lemma: "zasrati", addition: "", transitive: true, imperfective: false },
    ],
    "zasråmiti" => &[
        VerbDictionaryEntry { lemma: "zasråmiti", addition: "", transitive: true, imperfective: false },
    ],
    "zastariti" => &[
        VerbDictionaryEntry { lemma: "zastariti", addition: "", transitive: true, imperfective: false },
    ],
    "zastarjati" => &[
        VerbDictionaryEntry { lemma: "zastarjati", addition: "", transitive: true, imperfective: true },
    ],
    "zastati" => &[
        VerbDictionaryEntry { lemma: "zastati", addition: "(zastane)", transitive: true, imperfective: false },
    ],
    "zastavati" => &[
        VerbDictionaryEntry { lemma: "zastavati", addition: "", transitive: true, imperfective: true },
    ],
    "zastaviti" => &[
        VerbDictionaryEntry { lemma: "zastaviti", addition: "", transitive: true, imperfective: false },
    ],
    "zastavjati" => &[
        VerbDictionaryEntry { lemma: "zastavjati", addition: "", transitive: true, imperfective: true },
    ],
    "zastrašati" => &[
        VerbDictionaryEntry { lemma: "zastrašati", addition: "", transitive: true, imperfective: true },
    ],
    "zastrašiti" => &[
        VerbDictionaryEntry { lemma: "zastrašiti", addition: "", transitive: true, imperfective: false },
    ],
    "zastrěliti" => &[
        VerbDictionaryEntry { lemma: "zastrěliti", addition: "", transitive: true, imperfective: false },
    ],
    "zastrěljati" => &[
        VerbDictionaryEntry { lemma: "zastrěljati", addition: "", transitive: true, imperfective: true },
    ],
    "zastųpati" => &[
        VerbDictionaryEntry { lemma: "zastųpati", addition: "", transitive: true, imperfective: true },
    ],
    "zastųpiti" => &[
        VerbDictionaryEntry { lemma: "zastųpiti", addition: "", transitive: true, imperfective: false },
    ],
    "zasvěćati" => &[
        VerbDictionaryEntry { lemma: "zasvěćati", addition: "", transitive: true, imperfective: true },
    ],
    "zasějati" => &[
        VerbDictionaryEntry { lemma: "zasějati", addition: "(zasěje)", transitive: true, imperfective: false },
    ],
    "zatelefonovati" => &[
        VerbDictionaryEntry { lemma: "zatelefonovati", addition: "", transitive: true, imperfective: false },
    ],
    "zatknųti" => &[
        VerbDictionaryEntry { lemma: "zatknųti", addition: "", transitive: true, imperfective: false },
    ],
    "zatopiti" => &[
        VerbDictionaryEntry { lemma: "zatopiti", addition: "", transitive: true, imperfective: false },
    ],
    "zatrimati" => &[
        VerbDictionaryEntry { lemma: "zatrimati", addition: "", transitive: true, imperfective: false },
    ],
    "zatrimyvati" => &[
        VerbDictionaryEntry { lemma: "zatrimyvati", addition: "", transitive: true, imperfective: true },
    ],
    "zatręsti" => &[
        VerbDictionaryEntry { lemma: "zatręsti", addition: "", transitive: true, imperfective: false },
    ],
    "zatvarjati" => &[
        VerbDictionaryEntry { lemma: "zatvarjati", addition: "", transitive: true, imperfective: true },
    ],
    "zatvoriti" => &[
        VerbDictionaryEntry { lemma: "zatvoriti", addition: "", transitive: true, imperfective: false },
    ],
    "zatykati" => &[
        VerbDictionaryEntry { lemma: "zatykati", addition: "(zatyče)", transitive: true, imperfective: true },
    ],
    "zatėmniti" => &[
        VerbDictionaryEntry { lemma: "zatėmniti", addition: "", transitive: true, imperfective: false },
    ],
    "zatėmnjati" => &[
        VerbDictionaryEntry { lemma: "zatėmnjati", addition: "", transitive: true, imperfective: true },
    ],
    "zaustaviti" => &[
        VerbDictionaryEntry { lemma: "zaustaviti", addition: "", transitive: true, imperfective: false },
    ],
    "zaustavjati" => &[
        VerbDictionaryEntry { lemma: "zaustavjati", addition: "", transitive: true, imperfective: true },
    ],
    "zautrakati" => &[
        VerbDictionaryEntry { lemma: "zautrakati", addition: "", transitive: true, imperfective: true },
    ],
    "zauvažati" => &[
        VerbDictionaryEntry { lemma: "zauvažati", addition: "", transitive: true, imperfective: true },
    ],
    "zauvažiti" => &[
        VerbDictionaryEntry { lemma: "zauvažiti", addition: "", transitive: true, imperfective: false },
    ],
    "zavaliti" => &[
        VerbDictionaryEntry { lemma: "zavaliti", addition: "", transitive: true, imperfective: false },
    ],
    "zaviděti" => &[
        VerbDictionaryEntry { lemma: "zaviděti", addition: "(zavidi)", transitive: true, imperfective: true },
    ],
    "zavladnųti" => &[
        VerbDictionaryEntry { lemma: "zavladnųti", addition: "", transitive: true, imperfective: false },
    ],
    "zavladyvati" => &[
        VerbDictionaryEntry { lemma: "zavladyvati", addition: "", transitive: true, imperfective: true },
    ],
    "zavojevati" => &[
        VerbDictionaryEntry { lemma: "zavojevati", addition: "", transitive: true, imperfective: false },
    ],
    "zavojevyvati" => &[
        VerbDictionaryEntry { lemma: "zavojevyvati", addition: "", transitive: true, imperfective: true },
    ],
    "zavraćati" => &[
        VerbDictionaryEntry { lemma: "zavraćati", addition: "", transitive: true, imperfective: true },
    ],
    "zavråtiti" => &[
        VerbDictionaryEntry { lemma: "zavråtiti", addition: "", transitive: true, imperfective: false },
    ],
    "zavyti" => &[
        VerbDictionaryEntry { lemma: "zavyti", addition: "", transitive: true, imperfective: false },
    ],
    "zavęzati" => &[
        VerbDictionaryEntry { lemma: "zavęzati", addition: "(zavęže)", transitive: true, imperfective: false },
    ],
    "zavęzyvati" => &[
        VerbDictionaryEntry { lemma: "zavęzyvati", addition: "", transitive: true, imperfective: true },
    ],
    "zavědyvati" => &[
        VerbDictionaryEntry { lemma: "zavědyvati", addition: "", transitive: true, imperfective: true },
    ],
    "zavěćati" => &[
        VerbDictionaryEntry { lemma: "zavěćati", addition: "", transitive: true, imperfective: true },
    ],
    "zavŕtěti" => &[
        VerbDictionaryEntry { lemma: "zavŕtěti", addition: "", transitive: true, imperfective: false },
    ],
    "zavŕšati" => &[
        VerbDictionaryEntry { lemma: "zavŕšati", addition: "", transitive: true, imperfective: true },
    ],
    "zavŕšiti" => &[
        VerbDictionaryEntry { lemma: "zavŕšiti", addition: "", transitive: true, imperfective: false },
    ],
    "začarovati" => &[
        VerbDictionaryEntry { lemma: "začarovati", addition: "", transitive: true, imperfective: false },
    ],
    "začinati" => &[
        VerbDictionaryEntry { lemma: "začinati", addition: "", transitive: true, imperfective: true },
    ],
    "začuditi" => &[
        VerbDictionaryEntry { lemma: "začuditi", addition: "", transitive: true, imperfective: false },
    ],
    "začęti" => &[
        VerbDictionaryEntry { lemma: "začęti", addition: "(začne)", transitive: true, imperfective: false },
    ],
    "zašifrovati" => &[
        VerbDictionaryEntry { lemma: "zašifrovati", addition: "", transitive: true, imperfective: false },
    ],
    "zašiti" => &[
        VerbDictionaryEntry { lemma: "zašiti", addition: "(zašije)", transitive: true, imperfective: false },
    ],
    "zaštopati" => &[
        VerbDictionaryEntry { lemma: "zaštopati", addition: "", transitive: true, imperfective: false },
    ],
    "zaščititi" => &[
        VerbDictionaryEntry { lemma: "zaščititi", addition: "", transitive: true, imperfective: false },
    ],
    "zaščićati" => &[
        VerbDictionaryEntry { lemma: "zaščićati", addition: "", transitive: true, imperfective: true },
    ],
    "zaťmiti" => &[
        VerbDictionaryEntry { lemma: "zaťmiti", addition: "", transitive: true, imperfective: false },
    ],
    "zaťměvati" => &[
        VerbDictionaryEntry { lemma: "zaťměvati", addition: "", transitive: true, imperfective: true },
    ],
    "zažartovati" => &[
        VerbDictionaryEntry { lemma: "zažartovati", addition: "", transitive: true, imperfective: false },
    ],
    "zažegti" => &[
        VerbDictionaryEntry { lemma: "zažegti", addition: "(zažže)", transitive: true, imperfective: false },
    ],
    "zažigati" => &[
        VerbDictionaryEntry { lemma: "zažigati", addition: "", transitive: true, imperfective: true },
    ],
    "zdråvěti" => &[
        VerbDictionaryEntry { lemma: "zdråvěti", addition: "", transitive: true, imperfective: true },
    ],
    "zirkati" => &[
        VerbDictionaryEntry { lemma: "zirkati", addition: "", transitive: true, imperfective: true },
    ],
    "zirknųti" => &[
        VerbDictionaryEntry { lemma: "zirknųti", addition: "", transitive: true, imperfective: false },
    ],
    "zlobiti" => &[
        VerbDictionaryEntry { lemma: "zlobiti", addition: "", transitive: true, imperfective: true },
    ],
    "zloupotrěbiti" => &[
        VerbDictionaryEntry { lemma: "zloupotrěbiti", addition: "", transitive: true, imperfective: false },
    ],
    "zloupotrěbjati" => &[
        VerbDictionaryEntry { lemma: "zloupotrěbjati", addition: "", transitive: true, imperfective: true },
    ],
    "zlåtiti" => &[
        VerbDictionaryEntry { lemma: "zlåtiti", addition: "", transitive: true, imperfective: true },
    ],
    "znamenovati" => &[
        VerbDictionaryEntry { lemma: "znamenovati", addition: "", transitive: true, imperfective: true },
    ],
    "znati" => &[
        VerbDictionaryEntry { lemma: "znati", addition: "", transitive: true, imperfective: true },
    ],
    "značiti" => &[
        VerbDictionaryEntry { lemma: "značiti", addition: "", transitive: true, imperfective: true },
    ],
    "zrěti" => &[
        VerbDictionaryEntry { lemma: "zrěti", addition: "(zri)", transitive: true, imperfective: true },
        VerbDictionaryEntry { lemma: "zrěti", addition: "(zrěje)", transitive: true, imperfective: true },
    ],
    "zvati" => &[
        VerbDictionaryEntry { lemma: "zvati", addition: "(zȯve)", transitive: true, imperfective: true },
    ],
    "zvoniti" => &[
        VerbDictionaryEntry { lemma: "zvoniti", addition: "", transitive: true, imperfective: true },
    ],
    "zvěrěti" => &[
        VerbDictionaryEntry { lemma: "zvěrěti", addition: "", transitive: true, imperfective: true },
    ],
    "zvųčati" => &[
        VerbDictionaryEntry { lemma: "zvųčati", addition: "(zvųči)", transitive: true, imperfective: true },
    ],
    "zębti" => &[
        VerbDictionaryEntry { lemma: "zębti", addition: "", transitive: true, imperfective: true },
    ],
    "zějati" => &[
        VerbDictionaryEntry { lemma: "zějati", addition: "", transitive: true, imperfective: true },
    ],
    "zěvati" => &[
        VerbDictionaryEntry { lemma: "zěvati", addition: "", transitive: true, imperfective: true },
    ],
    "zěvnųti" => &[
        VerbDictionaryEntry { lemma: "zěvnųti", addition: "", transitive: true, imperfective: false },
    ],
    "čaditi" => &[
        VerbDictionaryEntry { lemma: "čaditi", addition: "", transitive: true, imperfective: true },
    ],
    "čarovati" => &[
        VerbDictionaryEntry { lemma: "čarovati", addition: "", transitive: true, imperfective: true },
    ],
    "časovati" => &[
        VerbDictionaryEntry { lemma: "časovati", addition: "", transitive: true, imperfective: true },
    ],
    "čekati" => &[
        VerbDictionaryEntry { lemma: "čekati", addition: "", transitive: true, imperfective: true },
    ],
    "česati" => &[
        VerbDictionaryEntry { lemma: "česati", addition: "(češe)", transitive: true, imperfective: true },
    ],
    "činiti" => &[
        VerbDictionaryEntry { lemma: "činiti", addition: "", transitive: true, imperfective: true },
    ],
    "čisliti" => &[
        VerbDictionaryEntry { lemma: "čisliti", addition: "", transitive: true, imperfective: true },
    ],
    "čistiti" => &[
        VerbDictionaryEntry { lemma: "čistiti", addition: "", transitive: true, imperfective: true },
    ],
    "čitati" => &[
        VerbDictionaryEntry { lemma: "čitati", addition: "", transitive: true, imperfective: true },
    ],
    "črniti" => &[
        VerbDictionaryEntry { lemma: "črniti", addition: "", transitive: true, imperfective: true },
    ],
    "črněti" => &[
        VerbDictionaryEntry { lemma: "črněti", addition: "", transitive: true, imperfective: true },
    ],
    "črpati" => &[
        VerbDictionaryEntry { lemma: "črpati", addition: "(črpe)", transitive: true, imperfective: true },
    ],
    "črstvěti" => &[
        VerbDictionaryEntry { lemma: "črstvěti", addition: "", transitive: true, imperfective: true },
    ],
    "črtati" => &[
        VerbDictionaryEntry { lemma: "črtati", addition: "", transitive: true, imperfective: true },
    ],
    "črveněti" => &[
        VerbDictionaryEntry { lemma: "črveněti", addition: "", transitive: true, imperfective: true },
    ],
    "čtiti" => &[
        VerbDictionaryEntry { lemma: "čtiti", addition: "", transitive: true, imperfective: true },
    ],
    "čuditi" => &[
        VerbDictionaryEntry { lemma: "čuditi", addition: "", transitive: true, imperfective: true },
    ],
    "čuti" => &[
        VerbDictionaryEntry { lemma: "čuti", addition: "", transitive: true, imperfective: true },
    ],
    "čuvati" => &[
        VerbDictionaryEntry { lemma: "čuvati", addition: "", transitive: true, imperfective: true },
    ],
    "čučati" => &[
        VerbDictionaryEntry { lemma: "čučati", addition: "", transitive: true, imperfective: true },
    ],
    "čėstitati" => &[
        VerbDictionaryEntry { lemma: "čėstitati", addition: "", transitive: true, imperfective: true },
    ],
    "čėstiti" => &[
        VerbDictionaryEntry { lemma: "čėstiti", addition: "", transitive: true, imperfective: true },
    ],
    "šalěti" => &[
        VerbDictionaryEntry { lemma: "šalěti", addition: "", transitive: true, imperfective: true },
    ],
    "šantažovati" => &[
        VerbDictionaryEntry { lemma: "šantažovati", addition: "", transitive: true, imperfective: true },
    ],
    "šifrovati" => &[
        VerbDictionaryEntry { lemma: "šifrovati", addition: "", transitive: true, imperfective: true },
    ],
    "širiti" => &[
        VerbDictionaryEntry { lemma: "širiti", addition: "", transitive: true, imperfective: true },
    ],
    "šiti" => &[
        VerbDictionaryEntry { lemma: "šiti", addition: "(šije)", transitive: true, imperfective: true },
    ],
    "škoditi" => &[
        VerbDictionaryEntry { lemma: "škoditi", addition: "", transitive: true, imperfective: true },
    ],
    "školiti" => &[
        VerbDictionaryEntry { lemma: "školiti", addition: "", transitive: true, imperfective: true },
    ],
    "šlepati" => &[
        VerbDictionaryEntry { lemma: "šlepati", addition: "", transitive: true, imperfective: true },
    ],
    "šlepnųti" => &[
        VerbDictionaryEntry { lemma: "šlepnųti", addition: "", transitive: true, imperfective: false },
    ],
    "šlifovati" => &[
        VerbDictionaryEntry { lemma: "šlifovati", addition: "", transitive: true, imperfective: true },
    ],
    "šokovati" => &[
        VerbDictionaryEntry { lemma: "šokovati", addition: "", transitive: true, imperfective: true },
    ],
    "štopati" => &[
        VerbDictionaryEntry { lemma: "štopati", addition: "", transitive: true, imperfective: true },
    ],
    "šuměti" => &[
        VerbDictionaryEntry { lemma: "šuměti", addition: "(šumi)", transitive: true, imperfective: true },
    ],
    "ščekotati" => &[
        VerbDictionaryEntry { lemma: "ščekotati", addition: "(ščekoče)", transitive: true, imperfective: true },
    ],
    "ščipati" => &[
        VerbDictionaryEntry { lemma: "ščipati", addition: "(ščipe)", transitive: true, imperfective: true },
    ],
    "ščipnųti" => &[
        VerbDictionaryEntry { lemma: "ščipnųti", addition: "", transitive: true, imperfective: false },
    ],
    "ščititi" => &[
        VerbDictionaryEntry { lemma: "ščititi", addition: "", transitive: true, imperfective: true },
    ],
    "ščęditi" => &[
        VerbDictionaryEntry { lemma: "ščęditi", addition: "", transitive: true, imperfective: true },
    ],
    "šėptati" => &[
        VerbDictionaryEntry { lemma: "šėptati", addition: "(šėpće)", transitive: true, imperfective: false },
    ],
    "žaliti" => &[
        VerbDictionaryEntry { lemma: "žaliti", addition: "", transitive: true, imperfective: true },
    ],
    "žariti" => &[
        VerbDictionaryEntry { lemma: "žariti", addition: "", transitive: true, imperfective: true },
    ],
    "žartovati" => &[
        VerbDictionaryEntry { lemma: "žartovati", addition: "", transitive: true, imperfective: true },
    ],
    "žegti" => &[
        VerbDictionaryEntry { lemma: "žegti", addition: "(žže)", transitive: true, imperfective: true },
    ],
    "želati" => &[
        VerbDictionaryEntry { lemma: "želati", addition: "", transitive: true, imperfective: true },
    ],
    "žiti" => &[
        VerbDictionaryEntry { lemma: "žiti", addition: "(žive)", transitive: true, imperfective: true },
    ],
    "žrti" => &[
        VerbDictionaryEntry { lemma: "žrti", addition: "(žre)", transitive: true, imperfective: true },
    ],
    "žrtvovati" => &[
        VerbDictionaryEntry { lemma: "žrtvovati", addition: "", transitive: true, imperfective: true },
    ],
    "žrěti" => &[
        VerbDictionaryEntry { lemma: "žrěti", addition: "(žre)", transitive: true, imperfective: true },
    ],
    "žuvati" => &[
        VerbDictionaryEntry { lemma: "žuvati", addition: "(žuje)", transitive: true, imperfective: true },
    ],
    "žužati" => &[
        VerbDictionaryEntry { lemma: "žužati", addition: "", transitive: true, imperfective: true },
    ],
    "žędati" => &[
        VerbDictionaryEntry { lemma: "žędati", addition: "", transitive: true, imperfective: true },
    ],
    "žęti" => &[
        VerbDictionaryEntry { lemma: "žęti", addition: "(žne)", transitive: true, imperfective: true },
    ],
    "žȯltěti" => &[
        VerbDictionaryEntry { lemma: "žȯltěti", addition: "", transitive: true, imperfective: true },
    ],
};

pub(crate) fn get_verbs(word: &str) -> Option<&'static [VerbDictionaryEntry]> {
    VERB_METADATA.get(word).copied()
}
