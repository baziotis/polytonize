import to_polytonic
import string_compare

def util(test):
  res = ''
  mono_text = test['mono']
  poly_corr = test['poly']
  
  poly_text = ''
  res = to_polytonic.to_polytonic(mono_text)
  for word, expl in res:
    if expl == None:
      if word in {'.', ',', ';'}:
        poly_text = poly_text[:-1]
        poly_text += word + ' '
      else:
        poly_text += word
      # END IF #
    else:
      poly_text += word + ' '
    # END IF #
  ### END FOR ###
  poly_text = poly_text.strip()
  assert string_compare.compare_strings(poly_text, poly_corr)
  
def test_long():
  t = {'mono': 'Πήγα να φάω, στον δρόμο προς την δουλειά, και είδα τον Γιώργο.',
       'poly': 'Πῆγα νὰ φάω, στὸν δρόμο πρὸς τὴν δουλειά, καὶ εῖδα τὸν Γιῶργο.'}
  
  util(t)

def test_case1():
  t = {'mono': 'έξαλλο είναι το αφεντικό.',
       'poly': 'έξαλλο εῖναι τὸ αφεντικό.'}
  
  util(t)

def test_case2():
  t = {'mono': 'τι είπες;',
       'poly': 'τί εῖπες;'}
  
  util(t)

def test_case3():
  t = {'mono': 'Η αστεία σειρά.',
       'poly': 'Η αστεία σειρά.'}
  
  util(t)

def test_case4():
  t = {'mono': 'Αυτό είπα.',
       'poly': 'Αυτὸ εῖπα.'}
  
  util(t)

def test_case5():
  t = {'mono': 'Είδαν την ταινία.',
       'poly': 'Εῖδαν τὴν ταινία.'}
  
  util(t)

def test_case6():
  t = {'mono': 'Tο χρώμα.',
       'poly': 'Tὸ χρῶμα.'}
  
  util(t)

def test_case7():
  t = {'mono': 'Αυτός κοιμάται.',
       'poly': 'Αυτὸς κοιμᾶται.'}
  
  util(t)
  
  util(t)

def test_case9():
  t = {'mono': 'Κάθε φορά.',
       'poly': 'Κάθε φορά.'}
  
  util(t)

def test_case10():
  t = {'mono': 'Αυτό είναι του διοικητή του.',
       'poly': 'Αυτὸ εῖναι τοῦ διοικητῆ του.'}
  
  util(t)

# -- This is gr_nlp_toolkit's problem. It tags Πήδα as Indicative,
# but it's Imperative. It seems to be a general problem. The next
# one is also tagged as Indicative. Also, in both cases the Person
# is wrong because it's '1' when it's 3rd. φεύγα is also incorrectly
# tagged as Past tense.
def test_case11():
  t = {'mono': 'Πήδα τον φράχτη.',
       'poly': 'Πήδα τὸν φράχτη.'}
  
  util(t)

def test_case12():
  t = {'mono': 'φεύγα από εδώ.',
       'poly': 'φεύγα απὸ εδῶ.'}
  
  util(t)

def test_case13():
  t = {'mono': 'Είδαν μια ταινία.',
       'poly': 'Εῖδαν μιὰ ταινία.'}
  
  util(t)

def test_case14():
  t = {'mono': 'Πήγα στήν δουλειά, αφού έφαγα.',
       'poly': 'Πῆγα στὴν δουλειά, αφοῦ έφαγα.'}
  
  util(t)

def test_case15():
  t = {'mono': 'Αυτή είναι ωραία.',
       'poly': 'Αυτὴ εῖναι ωραία.'}
  
  util(t)

# -- This is gr_nlp_toolkit's problem. It gives us:
# {'norm_word': 'ωραια', 'Case': 'Nom', 'Degree': '_', 'Gender': 'Fem', 'Number': 'Sing', 'POS': 'ADJ'}
# But this is not feminine here.
def test_case16():
  t = {'mono': 'Εδώ είναι ωραία.',
       'poly': 'Εδῶ εῖναι ωραῖα.'}
  
  util(t)

# It's a bit unclear what's correct in this.
def test_case18():
  t = {'mono': 'Καλώς τον.',
       'poly': 'Καλῶς τον.'}
  
  util(t)

{'norm_word': 'που', 'Case': 'Nom', 'Gender': 'Masc', 'Number': 'Sing', 'Person': '3', 'Poss': '_', 'PronType': 'Rel', 'POS': 'PRON'}
def test_anaforiko_pou():
  t = {'mono': 'Αυτός που πήγε στο γραφείο.',
       'poly': 'Αυτὸς ποὺ πῆγε στὸ γραφεῖο.'}

  util(t)

{'norm_word': 'που', 'Case': 'Nom', 'Gender': 'Masc', 'Number': 'Sing', 'Person': '3', 'Poss': '_', 'PronType': 'Rel', 'POS': 'PRON'}
{'norm_word': 'που', 'Case': 'Nom', 'Gender': 'Masc', 'Number': 'Sing', 'Person': '3', 'Poss': '_', 'PronType': 'Rel', 'POS': 'PRON'}
def test_erotimatiko_pou1():
  t = {'mono': 'Αυτός πού είναι;',
       'poly': 'Αυτὸς ποῦ εῖναι;'}
  
  util(t)
  
def test_erotimatiko_pou2():
  t = {'mono': 'Αυτός που είναι;',
       'poly': 'Αυτὸς ποῦ εῖναι;'}
  
  util(t)

# TODO: It's unclear whether gr_nlp_toolkit can help us differentiate between
# the two που (if there's no accent).
{'norm_word': 'που', 'Case': '_', 'Gender': '_', 'Number': 'Sing', 'Person': '3', 'Poss': '_', 'PronType': '_', 'POS': 'PRON'}
def test_erotimatiko_pou3():
  t = {'mono': 'Που είσαι;',
       'poly': 'Ποῦ εῖσαι;'}

  util(t)

def test_clitics():
  t = {'mono': "σας είπα ότι είναι δικό σας.",
       'poly': "σᾶς εῖπα ότι εῖναι δικό σας."}
  util(t)

def test_clitics2():
  t = {'mono': "να σας βάλω.",
       'poly': "νὰ σᾶς βάλω."}
  util(t)

def test_real_sentence1():
  t = {'mono': \
"""Αγαπητέ κύριε Μητσοτάκη, σαν παλιός συμμαθητής και ασήμαντος απλός πολίτης θέλω δημόσια να σας δώσω συγχαρητήρια και για την εκλογή στην ηγεσία της ΝΔ και γιατί πληροφορήθηκα ότι έχετε το καλύτερο βιογραφικό της χώρας.""",
'poly': \
"""Αγαπητὲ κύριε Μητσοτάκη, σὰν παλιὸς συμμαθητὴς καὶ ασήμαντος απλὸς πολίτης θέλω δημόσια νὰ σᾶς δώσω συγχαρητήρια καὶ γιὰ τὴν εκλογὴ στὴν ηγεσία τῆς ΝΔ καὶ γιατί πληροφορήθηκα ότι έχετε τὸ καλύτερο βιογραφικὸ τῆς χώρας."""
}
  util(t)
  
# TODO: Test ὡς and ὥς

if __name__ == '__main__':
  pass
  # test_anaforiko_pou()
  # test_erotimatiko_pou()