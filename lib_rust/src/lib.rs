extern crate core;

mod extension;
mod pos;
mod syllables;
mod test_common;

use crate::extension::{
  AccentExtensions, DiphthongExtensions, GrammarExtensions, NormalizationExtensions,
  VowelExtensions,
};
use crate::pos::{POS_MODULE, POS_REFINEMENTS, PartOfSpeech, PartOfSpeechType};
use crate::syllables::GreekSyllables;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use strum::EnumIs;

static TOKENIZER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\w+|[^\w\s]").unwrap());

static KNOWN_CIRCUMFLEXED: Lazy<HashSet<&str>> = Lazy::new(|| {
  HashSet::from([
    "γη",
    "γης",
    "νους",
    "πυρ",
    "φως",
    "δρυς",
    "δρυν",
    "μυς",
    "μυ",
    "μυν",
    "πας",
    "παν",
    "εμας",
    "εσας",
    "παντου",
    "πανταχου",
    "εδω",
    "αφου",
    "αφου",
  ])
});

static ALWAYS_ACUTE: Lazy<HashSet<&str>> = Lazy::new(|| HashSet::from(["τι", "γιατι", "ουτε"]));

static CLITICS: Lazy<HashSet<&str>> =
  Lazy::new(|| HashSet::from(["μου", "σου", "του", "της", "μας", "σας", "τους"]));

static CIRCUM_PRONOUNS: Lazy<HashSet<&str>> =
  Lazy::new(|| HashSet::from(["μου", "σου", "του", "της", "μας", "σας", "των"]));

static KNOWN_NON_ACCENTED_SINGLE_SYLLABLE: Lazy<HashSet<&str>> =
  Lazy::new(|| HashSet::from(["ο", "η", "αι", "οι"]));

static SINGLE_SYL_NON_ACCENTED_JOINED: Lazy<String> = Lazy::new(|| {
  KNOWN_NON_ACCENTED_SINGLE_SYLLABLE
    .iter()
    .copied()
    .collect::<Vec<_>>()
    .join(", ")
});

static CIRCUM_PRONOUNS_JOINED: Lazy<String> = Lazy::new(|| {
  CIRCUM_PRONOUNS
    .iter()
    .copied()
    .collect::<Vec<_>>()
    .join(", ")
});

static KNOWN_CIRCUMFLEXED_JOINED: Lazy<String> = Lazy::new(|| {
  KNOWN_CIRCUMFLEXED
    .iter()
    .copied()
    .collect::<Vec<_>>()
    .join(", ")
});

#[inline]
pub fn tokenize_greek_with_punctuation(text: &str) -> Vec<&str> {
  TOKENIZER_REGEX
    .find_iter(text)
    .map(|mat| mat.as_str())
    .collect()
}

#[derive(Debug, Default)]
pub struct PolytonizeContext<'c> {
  pub next_word: Option<&'c str>,
  pub next_word_pos: Option<&'c PartOfSpeech>,
  pub previous_was_double_accented: bool,
}

#[derive(Debug)]
pub enum Explanation {
  CliticFollowedByDoubleAccent,
  DoubleAccentFollowsClitic,
  KnownCircumflexed,
  CircumflexedPronoun,
  SingleSyllableWithAccent,
  SingleSyllableWithoutAccent,
  AntepenultAlwaysAccented,
  LongBeforeShort,
  ShortDiphthongAtUlt,
  AlphaAtNeutralUltima,
  AlphaAtFeminineUltima,
  AlphaAtVerbUltima,
  LongAlphaAtVerbalPenult,
  ShortAlphaAtVerbalPenult,
  AlwaysShort(String),
  AlwaysLong(String),
  LongDiphthong(String),
}

impl Display for Explanation {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let res = match self {
      Explanation::CliticFollowedByDoubleAccent => Cow::Borrowed(
        "Αὐτὴ ἡ λέξη εἶναι ἐγκλιτικὴ καὶ δὲν τονίζεται γιατί τὴν σκεφτόμαστε ὡς μέρος τῆς προηγουμένης λέξης.",
      ),
      Explanation::DoubleAccentFollowsClitic => Cow::Borrowed(
        "Ἡ ἐπόμενη λέξη εἶναι ἐγκλιτική, καὶ αὐτὴ τονίζεται στὴν προπαραλήγουσα. Γιὰ νὰ μὴν παραβιαστῇ ὁ νόμος τῆς τριχρονίας, μιᾶς καὶ ἡ ἐγκλιτικὴ λειτουργεῖ ὡς μία ἀκόμη συλλαβή, μπαίνει δεύτερος τόνος στὴν λήγουσα.",
      ),
      Explanation::KnownCircumflexed => Cow::Owned(format!(
        "Αὐτὴ ἡ λέξη ἀνήκει σὲ μιὰ μικρὴ λίστα λέξεων ποὺ παίρνουν πάντοτε περισπωμένη. Ἡ λίστα περιλαμβάνει τὶς ἐξῆς λέξεις: {}",
        *KNOWN_CIRCUMFLEXED_JOINED
      )),
      Explanation::CircumflexedPronoun => Cow::Owned(format!(
        "Ἡ συγκεκριμένη ἀντωνυμία ἀνήκει σὲ μιὰ λίστα ἀντωνυμιῶν ποὺ παίρνουν περισπωμένη (ὅταν τονίζονται). Ἡ πλήρης λίστα εἶναι ἡ ἐξῆς: {}",
        *CIRCUM_PRONOUNS_JOINED
      )),
      Explanation::SingleSyllableWithAccent => {
        Cow::Borrowed("Ἀνήκει στὸν γενικὸ κανόνα πὼς οἱ μονοσύλλαβες λέξεις τονίζονται μὲ ὀξεῖα.")
      }
      Explanation::SingleSyllableWithoutAccent => Cow::Owned(format!(
        "Αυτὴ ἡ λέξη εἶναι στὶς λίγες ἐξαιρέσεις μονοσύλλαβων λέξεων ποὺ δὲν τονίζονται. Αυτὲς οἱ εξαιρέσεις περιλαμβάνουν τὶς ἐξῆς λέξεις: {}",
        *SINGLE_SYL_NON_ACCENTED_JOINED
      )),
      Explanation::AntepenultAlwaysAccented => {
        Cow::Borrowed("Ἡ προπαραλήγουσα παίρνει πάντοτε ὁξεῖα.")
      }
      Explanation::LongBeforeShort => Cow::Borrowed("Μακρὸ πρὸ βραχέου περισπᾶται."),
      Explanation::AlwaysShort(syl) => Cow::Owned(format!("Τὸ «{syl}» εἶναι πάντοτε βραχύ.")),
      Explanation::ShortDiphthongAtUlt => Cow::Borrowed(
        "Ὅταν τὸ «αι» ἢ «οι» βρίσκονται στὴν λήγουσα, χωρίς κάποιο γράμμα μετὰ τὸ «ι», τότε εἶναι βραχέα.",
      ),
      Explanation::AlwaysLong(syl) => Cow::Owned(format!("Τὸ «{syl}» εἶναι πάντα μακρό.")),
      Explanation::LongDiphthong(syl) => Cow::Owned(format!(
        "Τὸ «{syl}» εἶναι δίφθογγος, καὶ (ὅπως στὶς περισσότερες περιπτώσεις) μακρὸ ἐδῶ."
      )),
      Explanation::AlphaAtNeutralUltima => {
        Cow::Borrowed("Tὸ «α» στὴν λήγουσα τῶν οὐδετέρων εἶναι βραχύ.")
      }
      Explanation::AlphaAtFeminineUltima => {
        Cow::Borrowed("Tὸ «α» στὴν λήγουσα τῶν θηλυκῶν εἶναι μακρό.")
      }
      Explanation::AlphaAtVerbUltima => Cow::Borrowed(
        "Τὸ «α» στὴν λήγουσα τῶν ρημάτων σὲ -α ἢ -αν εἶναι βραχύ, ἐκτὸς ἄν εἶναι προστακτική.",
      ),
      Explanation::LongAlphaAtVerbalPenult => Cow::Borrowed(
        "Τὸ «α» στὴν παραλήγουσα τῶν ρημαντικῶν καταλήξεων -άμε, -αμαι, -άτε, -άνε, -άσαι, -άται καὶ -άσθε/-άστε είναι μακρό.",
      ),
      Explanation::ShortAlphaAtVerbalPenult => Cow::Borrowed(
        "Τὰ βαρύτονα ρήματα ποὺ τελειώνουν σὲ -ζω (-άζω, -ίζω, -ύζω) ἔχουν τὸ δίχρονο τῆς παραλήγουσας βραχύ. Ἐξαιροῦνται τα «γρύζω», «κράζω».",
      ),
    };

    write!(f, "{res}")
  }
}

#[derive(Debug, Serialize)]
pub struct PolytonizedWord {
  pub word: String,
  pub explanation: Option<String>,
}

impl PolytonizedWord {
  pub fn new(word: String, explanation: Option<Explanation>) -> Self {
    Self {
      word,
      explanation: explanation.map(|explanation| explanation.to_string()),
    }
  }

  pub fn plain(word: String) -> Self {
    Self::new(word, None)
  }

  pub fn with_explanation(word: String, explanation: Explanation) -> Self {
    Self::new(word, Some(explanation))
  }
}

fn add_accent(s: &mut String, syllables: &GreekSyllables) {
  debug_assert_eq!(s, syllables.word);
  debug_assert_eq!(syllables.len(), 1);

  let (range, syl) = syllables.iter_syllables_with_positions().next().unwrap();

  let range = if syl.is_diphthong() {
    let first_char_len = syl.chars().next().unwrap().len_utf8();
    range.start + first_char_len..range.end
  } else {
    range
  };

  s.make_acute_range(range);
}

fn clitic_gets_accent(pos: &PartOfSpeech) -> bool {
  match &pos.ty {
    PartOfSpeechType::Pron(props) if props.poss.is_none() => true,
    PartOfSpeechType::Det(_) => true,
    _ => false,
  }
}

fn handle_clitic(
  word: &mut String,
  word_pos: &PartOfSpeech,
  syllables: &GreekSyllables,
  explanation: &mut Option<Explanation>,
) -> bool {
  let gets_accent = clitic_gets_accent(word_pos);

  if gets_accent {
    add_accent(word, syllables);
    if CIRCUM_PRONOUNS.contains(word_pos.normalized_word.as_str()) {
      word.make_circumflexed();
      *explanation = Some(Explanation::CircumflexedPronoun);
    }
  }

  gets_accent
}

#[derive(Debug, EnumIs)]
enum LongOrShort {
  Short,
  Long,
}

#[derive(Debug, EnumIs)]
enum SyllablePosition {
  Ultima,
  Penult,
}

fn long_or_short(
  word: &str,
  word_pos: &PartOfSpeech,
  syllable: &str,
  syllable_pos: SyllablePosition,
  explanation: &mut Option<Explanation>,
) -> Option<LongOrShort> {
  let syl_is_single_char = syllable.chars().count() == 1;
  let first_syllable_char = syllable.chars().next().unwrap();
  let short_ending_diphthongs = syllable == "οι" || syllable == "αι";
  let no_letter_follows = word.chars().last().is_some_and(|c| c == 'ι');
  let is_short_diphthong_at_ult =
    syllable_pos.is_ultima() && short_ending_diphthongs && no_letter_follows;

  let mut res = None;

  if syl_is_single_char && first_syllable_char.is_short_vowel() {
    *explanation = Some(Explanation::AlwaysShort(syllable.to_string()));
    res = Some(LongOrShort::Short);
  } else if is_short_diphthong_at_ult {
    *explanation = Some(Explanation::ShortDiphthongAtUlt);
    res = Some(LongOrShort::Short);
  } else if syl_is_single_char && first_syllable_char.is_long_vowel() {
    *explanation = Some(Explanation::AlwaysLong(syllable.to_string()));
    res = Some(LongOrShort::Long);
  } else if syllable.is_diphthong() {
    *explanation = Some(Explanation::LongDiphthong(syllable.to_string()));
    res = Some(LongOrShort::Long);
  } else {
    debug_assert!(syl_is_single_char);
    debug_assert!(first_syllable_char.is_mutable_vowel());

    match syllable_pos {
      SyllablePosition::Ultima => {
        if word_pos.normalized_word.ends_with('α') {
          let gender = word_pos.ty.gender();
          if gender.is_some_and(|g| g.is_neut()) {
            *explanation = Some(Explanation::AlphaAtNeutralUltima);
            res = Some(LongOrShort::Short);
          } else if gender.is_some_and(|g| g.is_fem()) {
            *explanation = Some(Explanation::AlphaAtFeminineUltima);
            res = Some(LongOrShort::Long);
          }
        }

        if word_pos.ty.is_verb()
          && (word_pos.normalized_word.ends_with('α') || word_pos.normalized_word.ends_with("αν"))
        {
          *explanation = Some(Explanation::AlphaAtVerbUltima);
          res = word_pos
            .ty
            .mood()
            .is_some_and(|m| m.is_imp())
            .then_some(LongOrShort::Long)
            .or(Some(LongOrShort::Short));
        }
      }
      SyllablePosition::Penult => {
        let long_suffixes = ["αμε", "αμαι", "άτε", "άνε", "ασαι", "αται", "ασθε", "αστε"];
        let short_suffixes = [
          "αζω",
          "ιζω",
          "υζω",
          "ασω",
          "ισω",
          "υσω",
          // ---- 2nd person singular ------
          "αζεις",
          "ιζεις",
          "υζει",
          "ασεις",
          "ισεις",
          "υσεις",
          // ---- 3rd person singular ------
          "αζει",
          "ιζει",
          "υζει",
          "ασει",
          "ισει",
          "υσει",
          // ---- 1st person plural ------
          "αζουμε",
          "ιζουμε",
          "υζουμε",
          "ασουμε",
          "ισουμε",
          "υσουμε",
          // ---- 2nd person plural ------
          "αζετε",
          "ιζετε",
          "υζετε",
          "ασετε",
          "ισετε",
          "υσετε",
          // ---- 3rd person plural ------
          "αζουν",
          "ιζουν",
          "υζουν",
          "ασουν",
          "ισουν",
          "υσουν",
        ];

        let prefixes = ["κρα", "γρυ"];

        if long_suffixes
          .iter()
          .any(|suffix| word_pos.normalized_word.ends_with(suffix))
        {
          *explanation = Some(Explanation::LongAlphaAtVerbalPenult);
          res = Some(LongOrShort::Long);
        } else if short_suffixes
          .iter()
          .any(|suffix| word_pos.normalized_word.ends_with(suffix))
          && !prefixes
            .iter()
            .any(|prefix| word_pos.normalized_word.starts_with(prefix))
        {
          *explanation = Some(Explanation::ShortAlphaAtVerbalPenult);
          res = Some(LongOrShort::Short);
        }
      }
    }
  }

  res
}

fn should_circum_penult(
  word: &str,
  word_pos: &PartOfSpeech,
  ultima: &str,
  penult: &str,
  explanation: &mut Option<Explanation>,
) -> bool {
  let penult_normalized = penult.normalize_greek();

  if long_or_short(
    word,
    word_pos,
    &penult_normalized,
    SyllablePosition::Penult,
    explanation,
  )
  .is_some_and(|res| res.is_long())
  {
    let ult_normalized = ultima.normalize_greek();
    if long_or_short(
      word,
      word_pos,
      &ult_normalized,
      SyllablePosition::Ultima,
      explanation,
    )
    .is_some_and(|res| res.is_short())
    {
      *explanation = Some(Explanation::LongBeforeShort);
      return true;
    }
  }

  false
}

#[allow(clippy::if_same_then_else, clippy::needless_bool)]
fn should_circum_ultima(
  word: &str,
  word_pos: &PartOfSpeech,
  ultima: &str,
  explanation: &mut Option<Explanation>,
) -> bool {
  let ultima_normalized = ultima.normalize_greek();

  if word.ends_with("είς")
    && (word_pos.ty.is_noun() || word_pos.ty.is_pron() || word_pos.ty.is_num())
  {
    true
  } else if ultima_normalized == "ου"
    && (word_pos.ty.is_adj() || word_pos.ty.is_noun() || word_pos.ty.is_prop_n())
  {
    true
  } else if word.ends_with("ώς") && word_pos.ty.is_adv() {
    true
  } else if (word.ends_with("άς")
    || word.ends_with("ά")
    || word.ends_with("ής")
    || word.ends_with("ών"))
    && word_pos.ty.is_prop_n()
  {
    true
  } else if word_pos.ty.is_verb()
    && long_or_short(
      word,
      word_pos,
      &ultima_normalized,
      SyllablePosition::Ultima,
      explanation,
    )
    .is_some_and(|res| res.is_long())
  {
    true
  } else if (matches!(&word_pos.ty, PartOfSpeechType::Noun(props) if props.case.as_ref().is_some_and(|case| case.is_gen()))
    || matches!(&word_pos.ty, PartOfSpeechType::Adj(props) if props.case.as_ref().is_some_and(|case| case.is_gen())))
    && long_or_short(
      word,
      word_pos,
      &ultima_normalized,
      SyllablePosition::Ultima,
      explanation,
    )
    .is_some()
  {
    true
  } else {
    false
  }
}

pub fn polytonize_word(
  word: &str,
  word_pos: &PartOfSpeech,
  ctx: &mut PolytonizeContext,
) -> PolytonizedWord {
  let mut res_word = String::from(word);

  if ctx.previous_was_double_accented {
    debug_assert!(word.is_clitic());

    return PolytonizedWord::with_explanation(res_word, Explanation::CliticFollowedByDoubleAccent);
  }

  let syllables = GreekSyllables::new(word, &word_pos.normalized_word);
  let accent_info = syllables.accent_info();

  if accent_info
    .as_ref()
    .is_some_and(|info| info.accent_count == 2)
  {
    debug_assert!(ctx.next_word.is_none() || ctx.next_word.is_some_and(|w| w.is_clitic()));

    ctx.previous_was_double_accented = true;
    return PolytonizedWord::with_explanation(res_word, Explanation::DoubleAccentFollowsClitic);
  }

  if KNOWN_CIRCUMFLEXED.contains(word_pos.normalized_word.as_str()) {
    if syllables.len() == 1 {
      add_accent(&mut res_word, &syllables);
    }
    res_word.make_circumflexed();
    return PolytonizedWord::with_explanation(res_word, Explanation::KnownCircumflexed);
  }

  let mut should_circum = false;

  let mut inverted_accent_pos = accent_info
    .as_ref()
    .map(|info| syllables.len() - 1 - info.syllable_index);

  let mut res_explanation: Option<Explanation> = None;

  'outer: {
    if ALWAYS_ACUTE.contains(word_pos.normalized_word.as_str()) && syllables.len() == 1 {
      add_accent(&mut res_word, &syllables);
      break 'outer;
    }

    if accent_info.is_none() {
      if word.to_uppercase() == word {
        break 'outer;
      }

      if syllables.len() != 1 {
        debug_assert_eq!(syllables.len(), 2);
        debug_assert_eq!(&syllables[0], "ι");
        debug_assert!(&syllables[1] == "ο" || &syllables[1] == "α");

        let range = syllables.get_range_unchecked(1);
        res_word.make_acute_range(range);
        inverted_accent_pos = Some(0);
      } else if word.is_clitic() {
        handle_clitic(&mut res_word, word_pos, &syllables, &mut res_explanation);
      } else if !KNOWN_NON_ACCENTED_SINGLE_SYLLABLE.contains(word_pos.normalized_word.as_str()) {
        inverted_accent_pos = Some(0);
        add_accent(&mut res_word, &syllables);
        res_explanation = Some(Explanation::SingleSyllableWithAccent);
      } else {
        res_explanation = Some(Explanation::SingleSyllableWithoutAccent);
      }
    } else {
      // Safety: We are guaranteed to have an inverted accent position since accent_info is some here.
      let inv_acc_pos = inverted_accent_pos.unwrap();
      if inv_acc_pos == 2 {
        res_explanation = Some(Explanation::AntepenultAlwaysAccented);
      } else if inv_acc_pos == 1 {
        debug_assert!(syllables.len() > 1);

        let mut syl_iter = syllables.iter_syllables().rev();
        let ult = syl_iter.next().unwrap();
        let penult = syl_iter.next().unwrap();

        should_circum = should_circum_penult(word, word_pos, ult, penult, &mut res_explanation);
      } else {
        let ult = syllables.iter_syllables().next_back().unwrap();
        should_circum = should_circum_ultima(word, word_pos, ult, &mut res_explanation);
      }
    }
  }

  if !should_circum {
    if res_word.to_uppercase() != word && inverted_accent_pos.is_some_and(|position| position == 0)
    {
      let mut add_grave = true;
      let next_is_punct = match ctx.next_word {
        None => true,
        Some(next_word) => next_word.is_punctuation(),
      };

      if let Some((next_word, next_word_pos)) = ctx.next_word.zip(ctx.next_word_pos) {
        if next_word.is_clitic() {
          add_grave = clitic_gets_accent(next_word_pos);
        }
      }

      if !ALWAYS_ACUTE.contains(word_pos.normalized_word.as_str()) && !next_is_punct && add_grave {
        res_word.make_graved();
      }
    }
  } else {
    res_word.make_circumflexed();
  }

  ctx.previous_was_double_accented = false;

  PolytonizedWord::new(res_word, res_explanation)
}

fn refine_pos(words: &[&str], parts_of_speech: &mut [PartOfSpeech]) {
  for (word, word_pos) in words.iter().copied().zip(parts_of_speech.iter_mut()) {
    let word_lower = word.to_lowercase();
    if let Some(refinement) = POS_REFINEMENTS.get(word_lower.as_str()) {
      *word_pos = refinement.clone();
    }
  }
}

pub fn convert_to_polytonic(words: &[&str], mut pos: Vec<PartOfSpeech>) -> Vec<PolytonizedWord> {
  debug_assert_eq!(pos.len(), words.len());

  refine_pos(words, &mut pos);

  let mut res = Vec::with_capacity(words.len());

  let mut ctx = PolytonizeContext::default();

  for (index, (word, word_pos)) in words.iter().zip(pos.iter()).enumerate() {
    ctx.next_word = words.get(index + 1).copied();
    ctx.next_word_pos = pos.get(index + 1);

    if word_pos.ty.is_punct() {
      res.push(PolytonizedWord::plain(word.to_string()));
      ctx.previous_was_double_accented = false;
      continue;
    }

    let poly_res = polytonize_word(word, word_pos, &mut ctx);
    res.push(poly_res);
  }

  res
}

/// Processes the input `text` and returns a list of words that
/// are correctly polytonized along with explanations per word
/// that describe why this conversion happened.
pub fn polytonize_text(text: &str) -> Vec<PolytonizedWord> {
  let pos = pos::get_for_text(text).unwrap();
  let words = tokenize_greek_with_punctuation(text);
  convert_to_polytonic(&words, pos)
}

/// _Entrypoint_ of the library which performs initializations.
/// Basically, triggers the global `Lazy` objects to load.
/// If you don't call this function nothing will happen.
/// The objects will be initialized the first time they are accessed.
/// This is merely a way of triggering the initializations on startup
pub fn initialize() {
  let _ = &*TOKENIZER_REGEX;
  let _ = &*KNOWN_CIRCUMFLEXED;
  let _ = &*KNOWN_CIRCUMFLEXED_JOINED;
  let _ = &*ALWAYS_ACUTE;
  let _ = &*CLITICS;
  let _ = &*CIRCUM_PRONOUNS;
  let _ = &*CIRCUM_PRONOUNS_JOINED;
  let _ = &*KNOWN_NON_ACCENTED_SINGLE_SYLLABLE;
  let _ = &*SINGLE_SYL_NON_ACCENTED_JOINED;
  let _ = &*POS_REFINEMENTS;
  let _ = &*POS_MODULE;
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_common::setup_python_paths;
  use rstest::rstest;

  #[test]
  fn test_syllabify_greek_word_with_last_char_being_a_vowel() {
    let syllables = GreekSyllables::new("Καλησπέρα", "Καλησπερα");
    let expected = vec!["α", "η", "έ", "α"];
    let actual = syllables.iter_syllables().collect::<Vec<_>>();
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_syllabify_greek_word() {
    let syllables = GreekSyllables::new("Μήπως", "Μηπως");
    let expected = vec!["ή", "ω"];
    let actual = syllables.iter_syllables().collect::<Vec<_>>();
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_syllabify_greek_word_with_diphthong() {
    let syllables = GreekSyllables::new("Τέλεια", "Τελεια");
    let expected = vec!["έ", "ει", "α"];
    let actual = syllables.iter_syllables().collect::<Vec<_>>();
    assert_eq!(expected, actual);
  }

  #[rstest]
  #[case::long(
    "Πήγα να φάω, στον δρόμο προς την δουλειά, και είδα τον Γιώργο.",
    "Πῆγα νὰ φάω, στὸν δρόμο πρὸς τὴν δουλειά, καὶ εῖδα τὸν Γιῶργο."
  )]
  #[case("έξαλλο είναι το αφεντικό.", "έξαλλο εῖναι τὸ αφεντικό.")]
  #[case("τι είπες;", "τί εῖπες;")]
  #[case("Η αστεία σειρά.", "Η αστεία σειρά.")]
  #[case("Αυτό είπα.", "Αυτὸ εῖπα.")]
  #[case("Είδαν την ταινία.", "Εῖδαν τὴν ταινία.")]
  #[case("Tο χρώμα.", "Tὸ χρῶμα.")]
  #[case("Αυτός κοιμάται.", "Αυτὸς κοιμᾶται.")]
  #[case("Κάθε φορά.", "Κάθε φορά.")]
  #[case("Αυτό είναι του διοικητή του.", "Αυτὸ εῖναι τοῦ διοικητῆ του.")]
  #[case("Πήδα τον φράχτη.", "Πήδα τὸν φράχτη.")]
  #[case("φεύγα από εδώ.", "φεύγα απὸ εδῶ.")]
  #[case("Είδαν μια ταινία.", "Εῖδαν μιὰ ταινία.")]
  #[case("Πήγα στήν δουλειά, αφού έφαγα.", "Πῆγα στὴν δουλειά, αφοῦ έφαγα.")]
  #[case("Αυτή είναι ωραία.", "Αυτὴ εῖναι ωραία.")]
  #[case("Εδώ είναι ωραία.", "Εδῶ εῖναι ωραῖα.")]
  #[case("Καλώς τον.", "Καλῶς τον.")]
  #[case::anaforiko("Αυτός που πήγε στο γραφείο.", "Αυτὸς ποὺ πῆγε στὸ γραφεῖο.")]
  #[case::erotimatiko_pou("Αυτός πού είναι;", "Αυτὸς ποῦ εῖναι;")]
  #[case::erotimatiko_pou("Αυτός που είναι;", "Αυτὸς ποῦ εῖναι;")]
  #[case::erotimatiko_pou("Που είσαι;", "Ποῦ εῖσαι;")]
  #[case::clitics("σας είπα ότι είναι δικό σας.", "σᾶς εῖπα ότι εῖναι δικό σας.")]
  #[case::clitics("να σας βάλω.", "νὰ σᾶς βάλω.")]
  #[case::sentence(
    "Αγαπητέ κύριε Μητσοτάκη, σαν παλιός συμμαθητής και ασήμαντος απλός πολίτης θέλω δημόσια να σας δώσω συγχαρητήρια και για την εκλογή στην ηγεσία της ΝΔ και γιατί πληροφορήθηκα ότι έχετε το καλύτερο βιογραφικό της χώρας.",
    "Αγαπητὲ κύριε Μητσοτάκη, σὰν παλιὸς συμμαθητὴς καὶ ασήμαντος απλὸς πολίτης θέλω δημόσια νὰ σᾶς δώσω συγχαρητήρια καὶ γιὰ τὴν εκλογὴ στὴν ηγεσία τῆς ΝΔ καὶ γιατί πληροφορήθηκα ότι έχετε τὸ καλύτερο βιογραφικὸ τῆς χώρας."
  )]
  #[case::double_accent("Τα προβλήματά του.", "Τὰ προβλήματά του.")]
  fn test_polytonize_text(#[case] text: &str, #[case] expected: &str) {
    setup_python_paths();
    let res = polytonize_text(text);
    let expected_tokens = tokenize_greek_with_punctuation(expected);
    let res_tokens = res.iter().map(|w| w.word.as_str()).collect::<Vec<_>>();
    assert_eq!(res_tokens, expected_tokens);
  }
}
