from gr_nlp_toolkit import Pipeline
import ast

nlp = Pipeline("pos")

def get_pos_single_word(word):
  doc = nlp(word)

  token = doc.tokens[0]
  res = (token.feats | {'POS': token.upos})
  return res

def get_pos(text):
  doc = nlp(text)

  res = [{'norm_word': token.text} | (token.feats | {'POS': token.upos}) for token in doc.tokens]
  return res

# text = "Η μαμά μου μου έφτιαξε γλυκό."
# doc = nlp(text)
# for token in doc.tokens:
#   print(token.text)
#   print(token.upos)
#   print(token.feats)
#   print('---')