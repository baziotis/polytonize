from typing import Tuple, List


def compare_strings(output: str, correct: str) -> None:
  if output == correct:
    return True

  # Get the length of both strings
  len1, len2 = len(output), len(correct)
  max_len = max(len1, len2)
  
  print(output)

  # Print a header for the comparison
  print("\nCharacter by character comparison:")
  print(f"{'Position':<10} {'Test Str':<10} {'Corr. Str':<10} {'Match':<10}")
  print("-" * 40)

  # Compare each character
  diffs = []
  for i in range(max_len):
    c1 = output[i] if i < len1 else 'None'
    c2 = correct[i] if i < len2 else 'None'
    match = c1 == c2

    if not match:
      diffs.append((i, c1, c2))

    print(f"{i:<10} {c1:<10} {c2:<10} {match}")
  ### END FOR ###

  # Display the Unicode code points for differing characters
  print("\nUnicode code points for differences:")
  print(f"{'Position':<10} {'Test String':<15} {'Corr. String':<15}")
  print("-" * 40)

  for pos, c1, c2 in diffs:
    code1 = f"U+{ord(c1):04X}" if c1 != 'None' else 'None'
    code2 = f"U+{ord(c2):04X}" if c2 != 'None' else 'None'
    print(f"{pos:<10} {c1} ({code1}){' '*(6-len(c1))} {c2} ({code2})")
  ### END FOR ###
  
  return False
