from gr_nlp_toolkit import Pipeline

nlp = Pipeline("pos")


def get_for_single_word(word: str) -> dict[str, str]:
  doc = nlp(word)

  token = doc.tokens[0]
  res = token.feats | {'POS': token.upos, 'normalized': token.text}
  return res


def get_for_text(text: str) -> list[dict[str, str]]:
  doc = nlp(text)

  res = [token.feats | {'POS': token.upos,
                        'normalized': token.text} for token in doc.tokens]
  return res
