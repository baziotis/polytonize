import unicodedata
import part_of_speech
import re
import string
import json

# TODO: We're not handling diairesis.

# Constants
VOWELS = 'αεηιουω'
DIPHTHONGS = {'αι', 'ει', 'οι', 'υι', 'αυ', 'ευ', 'ηυ', 'ου'}
TO_CIRCUMFLEX = {
    'ά': 'ᾶ',
    'ή': 'ῆ',
    'ώ': 'ῶ',
    'ί': 'ῖ',
    'ύ': 'ῦ'
}
TO_GRAVE = {
    'ά': 'ὰ',
    'ή': 'ὴ',
    'ώ': 'ὼ',
    'ί': 'ὶ',
    'ύ': 'ὺ',
    'έ': 'ὲ',
    'ό': 'ὸ'
}
TO_ACUTE = {
  'α': 'ά',
  'η': 'ή',
  'ω': 'ώ',
  'ι': 'ί',
  'υ': 'ύ',
  'ε': 'έ',
  'ο': 'ό',
}
ACC_VOWELS = 'άέήίόύώ'
SHORT_VOWELS = {'ε', 'ο'}
LONG_VOWELS = {'η', 'ω'}
# Δίχρονα
MUTABLE_VOWELS = {'α', 'ι', 'υ'}
# TODO: May be incomplete but can't think of anything else rn.
CLITICS = {
  'μου', 'σου', 'του', 'της', 'μας', 'σας', 'τους'
}
CIRCUM_PRONOUNS = {
  'μου', 'σου', 'του', 'της', 'μας', 'σας', 'των'
}
KNOWN_NON_ACCENTED_SINGLE_SYLLABLE = {
  'ο', 'η', 'αι', 'οι'
}
# TODO: More
ALWAYS_ACUTE = {'τι', 'γιατι', "ουτε"}
NON_ACC_TO_STR = ', '.join(list(KNOWN_NON_ACCENTED_SINGLE_SYLLABLE))
KNOWN_CIRCUMFLEXED = {
    "γη", "γης", "νους", "πυρ", "φως", "δρυς", "δρυν", "μυς", "μυ", "μυν", "πας",
    "παν", "εμας", "εσας", "παντου", "πανταχου", "εδω", "αφου", "αφου"
}

# Utility functions


def is_vowel(char):
  return char in VOWELS

def is_diphthong(s):
  return s in DIPHTHONGS

def is_possibly_clitic(word: str):
  return word.lower() in CLITICS

def remove_greek_diacritics(word):
    # Normalize the string to NFD to separate characters from diacritics
    normalized = unicodedata.normalize('NFD', word)
    # Remove all combining characters
    unaccented = ''.join(char for char in normalized if not unicodedata.combining(char))
    return unaccented

def normalize_word(word):
  return remove_greek_diacritics(word).lower()

def syllabify_greek_word(word):
  # Handle empty string case
  if not word:
    return []

  norm_word = normalize_word(word)

  syllables = []
  starting_pos = []
  i = 0

  while i < len(norm_word):
    # Get current character in both normalized and original form
    current_char = norm_word[i]
    curr_is_vowel = is_vowel(current_char)

    # Safely get the original character
    orig_current_char = word[i] if i < len(word) else ''

    # If we're at the end of the word, add the final character if it's a vowel and break
    if i == len(norm_word) - 1:
      if curr_is_vowel and orig_current_char:
        syllables.append(orig_current_char)
        starting_pos.append(i)
      break

    # Safely get next character in both normalized and original form
    next_char = norm_word[i + 1] if i + 1 < len(norm_word) else None
    orig_next_char = word[i + 1] if i + 1 < len(word) else None

    if curr_is_vowel:
      # Handle diphthongs
      if (next_char and is_vowel(next_char) and
          is_diphthong(current_char + next_char) and
              orig_next_char):  # Ensure we have a valid original next char
        syllables.append(orig_current_char + orig_next_char)
        starting_pos.append(i)
        i += 2
      else:
        # Single vowel
        syllables.append(orig_current_char)
        starting_pos.append(i)
        i += 1
    else:
      # Handle consonants - just move to next character
      i += 1

  assert (len(syllables) == len(starting_pos))
  return syllables, starting_pos


def has_accent(syl):
  for char in syl:
    if char in ACC_VOWELS:
      return True
  return False


def find_accent(syllables):
  acc_pos = -1
  for i, syl in enumerate(syllables):
    if has_accent(syl):
      if acc_pos != -1:
        raise Exception("double accent")
      acc_pos = i
  return acc_pos


def use_circumflex(word):
  result = []
  for char in word:
    result.append(TO_CIRCUMFLEX.get(char, char))
  return ''.join(result)

def use_grave(word):
  result = []
  for char in word:
    result.append(TO_GRAVE.get(char, char))
  return ''.join(result)

def add_accent(word, syllables, starting_pos):
  # TODO: Deal with caps.
  assert len(syllables) == 1
  syl = syllables[0]
  pos = starting_pos[0]

  # Can't modify the string in place, so we turn it into a list.
  word_chars = list(word)

  if is_diphthong(syl):
    word_chars[pos+1] = TO_ACUTE[word_chars[pos+1]]
  else:
    word_chars[pos] = TO_ACUTE[word_chars[pos]]

  res = ''.join(word_chars)
  return res

# TODO: Check Greek punctuation like ano teleia or greek quotation marks.
def is_punctuation(token):
  return all(char in string.punctuation for char in token)

def tokenize_greek_with_punctuation(text):
  # Match words or punctuation marks
  tokens = re.findall(r'\w+|[^\w\s]', text, re.UNICODE)
  return tokens




_global_explanation = None

# TODO:
# This function returns 'long', 'short', *OR NONE*. When doing it manually, it's
# not always necessary to know if a syllable is long or short. For example,
# «φάω». It doesn't matter what «α» is: if it's short, it gets an acute because
# it's short; if it's long, it gets an acute because it's long before long.
# --
# We should revisit this decision! Ideally we'd like to know whether it's long
# or short in all cases, and we should be able to figure it out in all cases.
def amend_explanation(amendment):
  if amendment is None:
    return
  global _global_explanation
  if _global_explanation == None:
    _global_explanation = amendment
  else:
    _global_explanation += " " + amendment

def long_or_short(syl_bundle, pos):
  syl = syl_bundle['syl']
  position = syl_bundle['pos']
  word = syl_bundle['word']
  norm_word = normalize_word(word)

  is_short_vowel = syl in SHORT_VOWELS
  # if we have 'οι' or 'αι' at the ultimate without a letter following them.
  is_ult = (position == 0)
  short_ending_dipths = (syl == 'οι' or syl == 'αι')
  no_letter_follows = (word[-1] == 'ι')
  is_short_diphthong_at_ult = is_ult and short_ending_dipths and no_letter_follows

  expl = None
  res = None
  if is_short_vowel:
    res = 'short'
    expl = f'Τὸ «{syl}» εἶναι πάντοτε βραχύ.'
  elif is_short_diphthong_at_ult:
    res = 'short'
    expl = f'Ὅταν τὸ «αι» ἢ «οι» βρίσκονται στὴν λήγουσα, χωρίς κάποιο γράμμα μετὰ τὸ «ι», τότε εἶναι βραχέα.'
  elif syl in LONG_VOWELS:
    res = 'long'
    expl = f'Τὸ «{syl}» εἶναι πάντα μακρό.'
  elif is_diphthong(syl):
    res = 'long'
    expl = f'Τὸ «{syl}» εἶναι δίφθογγος, καὶ (ὅπως στὶς περισσότερες περιπτώσεις) μακρὸ ἐδῶ.'
  else:
    assert syl in MUTABLE_VOWELS
    if position == 0:
      if norm_word.endswith('α'):
        if pos['Gender'] == 'Neut':
          res = 'short'
          expl = 'Tὸ «α» στὴν λήγουσα τῶν οὐδετέρων εἶναι βραχύ.'
        elif pos['Gender'] == 'Fem':
          res = 'long'
          # TODO: There are exceptions to this.
          expl = 'Tὸ «α» στὴν λήγουσα τῶν θηλυκῶν εἶναι μακρό.'
        # END IF #
      # END IF #
      
      if pos['POS'] == 'VERB' and (norm_word.endswith('α') or norm_word.endswith('αν')):
        expl = 'Τὸ «α» στὴν λήγουσα τῶν ρημάτων σὲ -α ἢ -αν εἶναι βραχύ, ἐκτὸς ἄν εἶναι προστακτική.'
        res = 'long' if pos['Mood'] == 'Imp' else 'short'
    elif position == 1:
      if norm_word.endswith(('αμε', 'αμαι', 'άτε', 'άνε', 'ασαι', 'αται', 'ασθε', 'αστε')):
        expl = 'Τὸ «α» στὴν παραλήγουσα τῶν ρημαντικῶν καταλήξεων -άμε, -αμαι, -άτε, -άνε, -άσαι, -άται καὶ -άσθε/-άστε είναι μακρό.'
        res = 'long'
    # END IF #
  # END IF #
  
  amend_explanation(expl)
  return res

def should_circum_ultima(word, pos, norm_ult):
  POS = pos['POS']
  ult_syl = norm_ult['syl']

  # TODO: It's unclear what's the best hierarchy of ifs.
  if word.endswith('είς'):
    if POS == 'NOUN' or POS == 'PRON' or POS == 'NUM':
      return True
  elif ult_syl == "ου":
    if POS == 'ADJ' or POS == 'NOUN' or POS == 'PROPN':
      # TODO: This is for all cases, so we need to look up the cases for a word.
      return True
  elif word.endswith('ώς'):
    if POS == 'ADV':
      return True
  elif (word.endswith('άς') or word.endswith('ά') or
        word.endswith('ής') or word.endswith('ών')):
    # TODO: This is also for all cases.
    # TODO: I think for -ης and -ων it's true for nouns too.
    if POS == 'PROPN':
      return True
  elif POS == 'VERB':
    if long_or_short(norm_ult, pos) == 'long':
      return True
  elif POS in ['NOUN', 'ADJ'] and pos['Case'] == 'Gen' and long_or_short(norm_ult, pos):
    return True
  return False


def should_circum_penult(word, pos, norm_ult, norm_penult):
  POS = pos['POS']
  ult_syl = norm_ult['syl']
  penult_syl = norm_penult['syl']

  if (long_or_short(norm_penult, pos) == 'long' and
      long_or_short(norm_ult, pos) == 'short'):
    amend_explanation("Μακρὸ πρὸ βραχέου περισπᾶται.")
    return True

  return False

def clitic_gets_accent(word, norm_word, pos, syllables, starting_pos):
  assert is_possibly_clitic(norm_word)
  gets_accent = False
  if pos['POS'] == 'PRON':
    if pos['Poss'] == '_':
      gets_accent = True
    # END IF #
  elif pos['POS'] == 'DET':
    gets_accent = True
  # END IF #

  if gets_accent:
    word = add_accent(word, syllables, starting_pos)
    if norm_word in CIRCUM_PRONOUNS:
      word = use_circumflex(word)
    # END IF #
    return word
  # END IF #

  return None
def polytonize_word(word, pos, next_word, next_pos):
  global _global_explanation
  res = ''
  _global_explanation = ''

  norm_word = normalize_word(word)
  syllables, starting_pos = syllabify_greek_word(word)
  print(f"{word}: {'-'.join(syllables)}")
  acc_pos = find_accent(syllables)
  should_circum = False
  inv_acc_pos = len(syllables) - 1 - acc_pos
  
  if norm_word in KNOWN_CIRCUMFLEXED:
    amend_explanation(f"Αὐτὴ ἡ λέξη ἀνήκει σὲ μιὰ μικρὴ λίστα λέξεων ποὺ παίρνουν πάντοτε περισπωμένη. Ἡ λίστα περιλαμβάνει τὶς ἐξῆς λέξεις: {", ".join(list(KNOWN_CIRCUMFLEXED))}")
    if len(syllables) == 1:
      word = add_accent(word, syllables, starting_pos)
      word = use_circumflex(word)
    # END IF #
    
    return word, _global_explanation
  # END IF #

  if norm_word in ALWAYS_ACUTE:
    if len(syllables) == 1:
      word = add_accent(word, syllables, starting_pos)
  elif acc_pos == -1:
    if word == word.upper():
      pass
    else:
      # Handle the special -ιο and -ια cases which syllabification splits into two
      # two syllables.
      if len(syllables) != 1:
        assert len(syllables) == 2
        assert syllables[0] == 'ι'
        assert syllables[1] in ['ο', 'α']
        word_chars = list(word)
        word_chars[starting_pos[1]] = TO_ACUTE[word_chars[starting_pos[1]]]
        word = ''.join(word_chars)
        inv_acc_pos = 0
      else:
        if is_possibly_clitic(word):
          clitic_word = clitic_gets_accent(word, norm_word, pos, syllables, starting_pos)
          if clitic_word is not None:
            word = clitic_word
        elif norm_word not in KNOWN_NON_ACCENTED_SINGLE_SYLLABLE:
          should_circum = False
          inv_acc_pos = 0
          word = add_accent(word, syllables, starting_pos)
        else:
          amend_explanation(\
  f"""
  Αυτὴ ἡ λέξη εἶναι στὶς λίγες ἐξαιρέσεις μονσύλλαβων λέξεων ποὺ δὲν τονίζονται.
  Αυτὲς οἱ εξαιρέσεις περιλαμβάνουν τὶς ἐξῆς λέξεις: {NON_ACC_TO_STR}.
  """)
        # END IF #
      # END IF #
    # END IF #
  else:
    # print(f"inv_acc_pos: {inv_acc_pos}")
    if inv_acc_pos > 2:
      raise Exception("accent is beyond the antepenult.")

    ult = syllables[-1]
    if inv_acc_pos == 2: # antepenult
      amend_explanation('Ἡ προπαραλήγουσα παίρνει πάντοτε ὁξεῖα.')
    else:
      norm_ult = {'syl': normalize_word(ult), 'pos': 0, 'word': word}
      if inv_acc_pos == 1: # penult
        assert len(syllables) > 1, "Expected more than one syllable"
        penult = syllables[-2]
        norm_penult = {'syl': normalize_word(
            penult), 'pos': 1, 'word': word}
        should_circum = should_circum_penult(word, pos, norm_ult, norm_penult)
      else: # ultima
        should_circum = should_circum_ultima(word, pos, norm_ult)
        # END IF #
      # END IF #
    # END IF #
  # END IF #

  if should_circum:
    res = use_circumflex(word)
  else:
    res = word
    if word != word.upper():
      # Check if we need to grave
      if inv_acc_pos == 0:
        # If there's no next word, we consider the sentence ended, so we assume a
        # period.
        next_is_punct = (next_word == None or is_punctuation(next_word))
        add_grave = True
        if next_word != None:
          if is_possibly_clitic(next_word):
            next_syllables, next_starting_pos = syllabify_greek_word(next_word)
            next_word_norm = normalize_word(next_word)
            clitic_word = clitic_gets_accent(next_word, next_word_norm, next_pos, next_syllables, next_starting_pos)
            if clitic_word is None:
              add_grave = False
            # END IF #
          # END IF #
        # END IF #

        if (norm_word not in ALWAYS_ACUTE) and (not next_is_punct) and (add_grave):
          res = use_grave(word)
      # END IF #
    # END IF #
  # END IF #
  
  return res, _global_explanation

def polytonize_single_word(word):

  # Get part of speech for the word
  pos = part_of_speech.get_pos_single_word(word)

  return polytonize_word(word, pos, None, None)

with open('cached_refinements.json', 'r') as fp:
  CACHED_REFINEMENTS = json.load(fp)

def refine_pos(orig_words, pos):
  # gr_nlp_toolkit makes mistakes. Here we try to catch some of them.
  assert len(orig_words) == len(pos)
  new_pos = []
  for orig_w, pos_w in zip(orig_words, pos):
    new_pos_w = pos_w
    # First we deal with simple cases where we can tell without any context, and
    # we cache them.
    lowered = orig_w.lower()
    try:
      new_pos_w = CACHED_REFINEMENTS[lowered]
    except:
      pass

    new_pos.append(new_pos_w)
  ### END FOR ###

  return new_pos

def to_polytonic(mono_text):
  # Pairs of (word, explanation)
  res = []
  pos = part_of_speech.get_pos(mono_text)

  orig_words = tokenize_greek_with_punctuation(mono_text)
  print(orig_words)
  pos = refine_pos(orig_words, pos)
  for pos_w in pos:
    print(json.dumps(pos_w, indent=2, ensure_ascii=False))
  assert (len(orig_words) == len(pos))
  i = 0
  len_orig_words = len(orig_words)
  len_pos = len(pos)
  while i < len_orig_words:
    orig_word = orig_words[i]
    next_word = orig_words[i+1] if i < (len_orig_words-1) else None
    next_pos = pos[i+1] if i < (len_pos-1) else None
    word_pos = pos[i]
    assert normalize_word(orig_word) == word_pos['norm_word']
    if word_pos['POS'] == 'PUNCT':
      assert is_punctuation(orig_word)
      res.append((orig_word, None))
    else:
      poly_word, expl = polytonize_word(orig_word, word_pos, next_word, next_pos)
      res.append((poly_word, expl))
    # END IF #
    i += 1
  ### END WHILE ###
  return res