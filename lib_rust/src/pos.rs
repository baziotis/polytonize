#![allow(dead_code)]

use once_cell::sync::Lazy;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;
use strum::{EnumIs, EnumString};

fn label_from_py_object<T: FromStr>(bound: &Bound<'_, PyAny>) -> PyResult<Option<T>> {
    let value: &str = bound.extract()?;
    match value {
        "_" => Ok(None),
        _ => Ok(T::from_str(value).ok()),
    }
}

#[derive(Debug, Copy, Clone, EnumString)]
pub enum AbbrLabels {
    Yes,
}

#[derive(Debug, Copy, Clone, EnumString)]
pub enum AspectLabels {
    Perf,
    Imp,
}

#[derive(Debug, Copy, Clone, EnumString, EnumIs)]
pub enum CaseLabels {
    Dat,
    Acc,
    Gen,
    Nom,
    Voc,
}

#[derive(Debug, Copy, Clone, EnumString)]
pub enum DefiniteLabels {
    Ind,
    Def,
}

#[derive(Debug, Copy, Clone, EnumString)]
pub enum DegreeLabels {
    Cmp,
    Sup,
}

#[derive(Debug, Copy, Clone, EnumString)]
pub enum ForeignLabels {
    Yes,
}

#[derive(Debug, Copy, Clone, EnumString, EnumIs)]
pub enum GenderLabels {
    Fem,
    Masc,
    Neut,
}

#[derive(Debug, Copy, Clone, EnumString, EnumIs)]
pub enum MoodLabels {
    Ind,
    Imp,
}

#[derive(Debug, Copy, Clone, EnumString)]
pub enum NumTypeLabels {
    Mult,
    Card,
    Ord,
    Sets,
}

#[derive(Debug, Copy, Clone, EnumString)]
pub enum NumberLabels {
    Sing,
    Plur,
}

#[derive(Debug, Copy, Clone, EnumString)]
pub enum PersonLabels {
    First,
    Second,
    Third,
}

#[derive(Debug, Copy, Clone, EnumString)]
pub enum PossLabels {
    Yes,
}

#[derive(Debug, Copy, Clone, EnumString)]
pub enum PronTypeLabels {
    Ind,
    Art,
    Rel,
    Dem,
    Prs,
    #[strum(serialize = "Ind,Rel")]
    IndRel,
    Int,
}

#[derive(Debug, Copy, Clone, EnumString)]
pub enum TenseLabels {
    Pres,
    Past,
}

#[derive(Debug, Copy, Clone, EnumString)]
pub enum VerbFormLabels {
    Part,
    Conv,
    Inf,
    Fin,
}

#[derive(Debug, Copy, Clone, EnumString)]
pub enum VoiceLabels {
    Pass,
    Act,
}

#[derive(Debug, Clone, FromPyObject)]
#[pyo3(from_item_all, rename_all = "PascalCase")]
pub struct XPosProps {
    #[pyo3(from_py_with = label_from_py_object)]
    pub foreign: Option<ForeignLabels>,
}

#[derive(Debug, Clone, FromPyObject)]
#[pyo3(from_item_all, rename_all = "PascalCase")]
pub struct PropNProps {
    #[pyo3(from_py_with = label_from_py_object)]
    pub number: Option<NumberLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub gender: Option<GenderLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub case: Option<CaseLabels>,
}

#[derive(Debug, Clone, FromPyObject)]
#[pyo3(from_item_all, rename_all = "PascalCase")]
pub struct PronProps {
    #[pyo3(from_py_with = label_from_py_object)]
    pub number: Option<NumberLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub gender: Option<GenderLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub person: Option<PersonLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub poss: Option<PossLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub pron_type: Option<PronTypeLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub case: Option<CaseLabels>,
}

#[derive(Debug, Clone, FromPyObject)]
#[pyo3(from_item_all, rename_all = "PascalCase")]
pub struct AdjProps {
    #[pyo3(from_py_with = label_from_py_object)]
    pub degree: Option<DegreeLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub number: Option<NumberLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub gender: Option<GenderLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub case: Option<CaseLabels>,
}

#[derive(Debug, Clone, FromPyObject)]
#[pyo3(from_item_all, rename_all = "PascalCase")]
pub struct AuxProps {
    #[pyo3(from_py_with = label_from_py_object)]
    pub mood: Option<MoodLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub aspect: Option<AspectLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub tense: Option<TenseLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub number: Option<NumberLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub person: Option<PersonLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub verb_form: Option<VerbFormLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub voice: Option<VoiceLabels>,
}

#[derive(Debug, Clone, FromPyObject)]
#[pyo3(from_item_all, rename_all = "PascalCase")]
pub struct AdvProps {
    #[pyo3(from_py_with = label_from_py_object)]
    pub degree: Option<DegreeLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub abbr: Option<AbbrLabels>,
}

#[derive(Debug, Clone, FromPyObject)]
#[pyo3(from_item_all, rename_all = "PascalCase")]
pub struct DetProps {
    #[pyo3(from_py_with = label_from_py_object)]
    pub number: Option<NumberLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub gender: Option<GenderLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub pron_type: Option<PronTypeLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub definite: Option<DefiniteLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub case: Option<CaseLabels>,
}

#[derive(Debug, Clone, FromPyObject)]
#[pyo3(from_item_all, rename_all = "PascalCase")]
pub struct NumProps {
    #[pyo3(from_py_with = label_from_py_object)]
    pub num_type: Option<NumTypeLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub number: Option<NumberLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub gender: Option<GenderLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub case: Option<CaseLabels>,
}

#[derive(Debug, Clone, FromPyObject)]
#[pyo3(from_item_all, rename_all = "PascalCase")]
pub struct NounProps {
    #[pyo3(from_py_with = label_from_py_object)]
    pub number: Option<NumberLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub gender: Option<GenderLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub abbr: Option<AbbrLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub case: Option<CaseLabels>,
}

#[derive(Debug, Clone, FromPyObject)]
#[pyo3(from_item_all, rename_all = "PascalCase")]
pub struct AdpProps {
    #[pyo3(from_py_with = label_from_py_object)]
    pub number: Option<NumberLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub gender: Option<GenderLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub case: Option<CaseLabels>,
}

#[derive(Debug, Clone, FromPyObject)]
#[pyo3(from_item_all, rename_all = "PascalCase")]
pub struct VerbProps {
    #[pyo3(from_py_with = label_from_py_object)]
    pub mood: Option<MoodLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub aspect: Option<AspectLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub tense: Option<TenseLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub number: Option<NumberLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub gender: Option<GenderLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub person: Option<PersonLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub verb_form: Option<VerbFormLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub voice: Option<VoiceLabels>,
    #[pyo3(from_py_with = label_from_py_object)]
    pub case: Option<CaseLabels>,
}

#[derive(Debug, Clone, EnumIs)]
pub enum PartOfSpeechType {
    X(XPosProps),
    PropN(PropNProps),
    Pron(PronProps),
    Adj(AdjProps),
    Aux(AuxProps),
    Adv(AdvProps),
    Det(DetProps),
    Num(NumProps),
    Noun(NounProps),
    Adp(AdpProps),
    Verb(VerbProps),
    Part,
    Sym,
    CConj,
    Punct,
    SConj,
    Unknown,
}

impl PartOfSpeechType {
    pub fn gender(&self) -> Option<GenderLabels> {
        match self {
            PartOfSpeechType::PropN(props) => props.gender,
            PartOfSpeechType::Pron(props) => props.gender,
            PartOfSpeechType::Adj(props) => props.gender,
            PartOfSpeechType::Det(props) => props.gender,
            PartOfSpeechType::Num(props) => props.gender,
            PartOfSpeechType::Noun(props) => props.gender,
            PartOfSpeechType::Adp(props) => props.gender,
            PartOfSpeechType::Verb(props) => props.gender,
            _ => None,
        }
    }

    pub fn mood(&self) -> Option<MoodLabels> {
        match self {
            PartOfSpeechType::Aux(props) => props.mood,
            PartOfSpeechType::Verb(props) => props.mood,
            _ => None,
        }
    }
}

impl FromPyObject<'_> for PartOfSpeechType {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let pos = ob.get_item("POS")?;
        let pos = pos.extract()?;
        Ok(match pos {
            "X" => PartOfSpeechType::X(XPosProps::extract_bound(ob)?),
            "PROPN" => PartOfSpeechType::PropN(PropNProps::extract_bound(ob)?),
            "PRON" => PartOfSpeechType::Pron(PronProps::extract_bound(ob)?),
            "ADJ" => PartOfSpeechType::Adj(AdjProps::extract_bound(ob)?),
            "AUX" => PartOfSpeechType::Aux(AuxProps::extract_bound(ob)?),
            "DET" => PartOfSpeechType::Det(DetProps::extract_bound(ob)?),
            "NUM" => PartOfSpeechType::Num(NumProps::extract_bound(ob)?),
            "NOUN" => PartOfSpeechType::Noun(NounProps::extract_bound(ob)?),
            "ADP" => PartOfSpeechType::Adp(AdpProps::extract_bound(ob)?),
            "VERB" => PartOfSpeechType::Verb(VerbProps::extract_bound(ob)?),
            "PART" => PartOfSpeechType::Part,
            "SYM" => PartOfSpeechType::Sym,
            "CCONJ" => PartOfSpeechType::CConj,
            "PUNCT" => PartOfSpeechType::Punct,
            "SCONJ" => PartOfSpeechType::SConj,
            _ => PartOfSpeechType::Unknown,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PartOfSpeech {
    pub ty: PartOfSpeechType,
    pub normalized_word: String,
}

impl FromPyObject<'_> for PartOfSpeech {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let normalized_word = ob.get_item("normalized")?.extract()?;
        let ty = ob.extract()?;

        Ok(Self {
            ty,
            normalized_word,
        })
    }
}

pub(crate) struct PartOfSpeechModule {
    get_for_single_word: Py<PyAny>,
    get_for_text: Py<PyAny>,
}

impl PartOfSpeechModule {
    pub(crate) fn new() -> Self {
        Python::with_gil(|py| {
            let pos_module = PyModule::import(py, "pos").unwrap();
            let get_for_single_word = pos_module.getattr("get_for_single_word").unwrap().unbind();
            let get_for_text = pos_module.getattr("get_for_text").unwrap().unbind();
            Self {
                get_for_single_word,
                get_for_text,
            }
        })
    }

    pub(crate) fn get_for_single_word(&self, word: &str) -> Option<PartOfSpeech> {
        Python::with_gil(|py| {
            self.get_for_single_word
                .call1(py, (word,))
                .ok()
                .and_then(|x| x.extract(py).ok())
        })
    }

    pub(crate) fn get_for_text(&self, text: &str) -> Option<Vec<PartOfSpeech>> {
        Python::with_gil(|py| {
            self.get_for_text
                .call1(py, (text,))
                .ok()
                .and_then(|res| res.extract(py).ok())
        })
    }
}

pub(crate) static POS_MODULE: Lazy<PartOfSpeechModule> = Lazy::new(PartOfSpeechModule::new);

pub(crate) static POS_REFINEMENTS: Lazy<HashMap<&str, PartOfSpeech>> = Lazy::new(|| {
    HashMap::from([
        (
            "πήδα",
            PartOfSpeech {
                normalized_word: "πηδα".to_string(),
                ty: PartOfSpeechType::Verb(VerbProps {
                    aspect: Some(AspectLabels::Perf),
                    case: None,
                    gender: None,
                    mood: Some(MoodLabels::Imp),
                    number: Some(NumberLabels::Sing),
                    person: Some(PersonLabels::Second),
                    tense: Some(TenseLabels::Past),
                    verb_form: Some(VerbFormLabels::Fin),
                    voice: Some(VoiceLabels::Act),
                }),
            },
        ),
        (
            "φεύγα",
            PartOfSpeech {
                normalized_word: "φευγα".to_string(),
                ty: PartOfSpeechType::Verb(VerbProps {
                    aspect: Some(AspectLabels::Perf),
                    case: None,
                    gender: None,
                    mood: Some(MoodLabels::Imp),
                    number: Some(NumberLabels::Sing),
                    person: Some(PersonLabels::Second),
                    tense: Some(TenseLabels::Past),
                    verb_form: Some(VerbFormLabels::Fin),
                    voice: Some(VoiceLabels::Act),
                }),
            },
        ),
        (
            "φύγετε",
            PartOfSpeech {
                normalized_word: "φυγετε".to_string(),
                ty: PartOfSpeechType::Verb(VerbProps {
                    aspect: Some(AspectLabels::Perf),
                    case: None,
                    gender: None,
                    mood: Some(MoodLabels::Imp),
                    number: Some(NumberLabels::Plur),
                    person: Some(PersonLabels::Second),
                    tense: Some(TenseLabels::Past),
                    verb_form: Some(VerbFormLabels::Fin),
                    voice: Some(VoiceLabels::Act),
                }),
            },
        ),
    ])
});

pub fn get_for_single_word(word: &str) -> Option<PartOfSpeech> {
    POS_MODULE.get_for_single_word(word)
}

pub fn get_for_text(text: &str) -> Option<Vec<PartOfSpeech>> {
    POS_MODULE.get_for_text(text)
}

#[cfg(test)]
mod tests {
    use crate::pos::PartOfSpeechType;
    use crate::test_common::setup_python_paths;
    use pyo3::prelude::*;

    #[test]
    fn text_extract_single_pos() {
        setup_python_paths();
        Python::with_gil(|py| {
            let pos_module = PyModule::import(py, "pos").unwrap();
            let _pos: PartOfSpeechType = pos_module
                .getattr("get_for_single_word")
                .unwrap()
                .call1(("Τρώω",))
                .unwrap()
                .extract()
                .unwrap();
        })
    }

    #[test]
    fn test_extract_pos_from_text() {
        setup_python_paths();
        Python::with_gil(|py| {
            let pos_module = PyModule::import(py, "pos").unwrap();
            let _pos: Vec<PartOfSpeechType> = pos_module
                .getattr("get_for_text")
                .unwrap()
                .call1(("Τρώω και μιλάω",))
                .unwrap()
                .extract()
                .unwrap();
        })
    }
}
