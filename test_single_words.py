import to_polytonic
import string_compare

def util(test):
  word = test['orig']
  poly_corr = test['poly']
  poly_word, _ = to_polytonic.polytonize_single_word(word)

  assert string_compare.compare_strings(poly_word, poly_corr)  

def test_anthropos():
  """person, human"""
  util({'orig': 'άνθρωπος', 'poly': 'άνθρωπος'})

def test_anthropous():
  """accusative of ἄνθρωπος"""
  util({'orig': 'ανθρώπους', 'poly': 'ανθρώπους'})

def test_kalimera():
  """good morning"""
  util({'orig': 'καλημέρα', 'poly': 'καλημέρα'})

def test_o():
  """masculine "the\""""
  util({'orig': 'ο', 'poly': 'ο'})

def test_thalassa():
  """sea"""
  util({'orig': 'θάλασσα', 'poly': 'θάλασσα'})

def test_oinos():
  """wine (in modern Greek it's usually "κρασί")"""
  util({'orig': 'οίνος', 'poly': 'οῖνος'})

def test_thymamai():
  """I remember"""
  util({'orig': 'θυμάμαι', 'poly': 'θυμᾶμαι'})

def test_oraioi():
  """beautiful (masculine plural)"""
  util({'orig': 'ωραίοι', 'poly': 'ωραῖοι'})

def test_einai():
  """is (3rd singular or plural)"""
  util({'orig': 'είναι', 'poly': 'εῖναι'})

def test_poleos():
  """city (genitive)"""
  util({'orig': 'πόλεως', 'poly': 'πόλεως'})

def test_idio():
  """same"""
  util({'orig': 'ίδιο', 'poly': 'ίδιο'})

def test_ellinika():
  """greek"""
  util({'orig': 'ελληνικά', 'poly': 'ελληνικά'})

def test_syngrafeis():
  """authors (both nom. and accusative)"""
  util({'orig': 'συγγραφείς', 'poly': 'συγγραφεῖς'})

def test_emeis():
  """we"""
  util({'orig': 'εμείς', 'poly': 'εμεῖς'})

def test_treis():
  """three"""
  util({'orig': 'τρείς', 'poly': 'τρεῖς'})

def test_iisous():
  """Jesus"""
  util({'orig': 'Ιησούς', 'poly': 'Ιησοῦς'})

def test_eftychos():
  """fortunately"""
  util({'orig': 'ευτυχώς', 'poly': 'ευτυχῶς'})

def test_loukas():
  """Luke"""
  util({'orig': 'Λουκάς', 'poly': 'Λουκᾶς'})

def test_thalis():
  """Thales"""
  util({'orig': 'Θαλής', 'poly': 'Θαλῆς'})

def test_xenofon():
  """Xenophon"""
  util({'orig': 'Ξενοφών', 'poly': 'Ξενοφῶν'})

def test_agapo():
  """I love"""
  util({'orig': 'αγαπώ', 'poly': 'αγαπῶ'})

def test_pira():
  """I took"""
  util({'orig': 'πήρα', 'poly': 'πῆρα'})

def test_eidan():
  """They saw"""
  util({'orig': 'είδαν', 'poly': 'εῖδαν'})

def test_douleia():
  """work (noun)"""
  util({'orig': 'δουλειά', 'poly': 'δουλειά'})

def test_syniltha():
  """recovered, came back to my senses"""
  util({'orig': 'συνήλθα', 'poly': 'συνῆλθα'})

def test_exallo():
  """furious - there's a problem with caps."""
  util({'orig': 'Έξαλλο', 'poly': 'Έξαλλο'})

def test_dimiourgous():
  """creators - not all -ούς get a circumflex. Only if it's -ους in the nominative."""
  util({'orig': 'δημιουργούς', 'poly': 'δημιουργούς'})

def test_oute():
  """neither - In such special cases of composites (in this case ου + τε),
  the rule of long before short doesn't hold."""
  util({'orig': 'ούτε', 'poly': 'ούτε'})

def test_nous():
  """mind"""
  util({'orig': 'νους', 'poly': 'νοῦς'})

def test_des():
  """look (imperative) - single syllable so it doesn't get an accent in monotonic but it should
  get an accent in polytonic."""
  util({'orig': 'δες', 'poly': 'δές'})
