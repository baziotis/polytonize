use crate::extension::{DiphthongExtensions, VowelExtensions};
use std::iter::Peekable;
use std::ops::{Index, Range};
use std::str::CharIndices;
use unicode_normalization::UnicodeNormalization;

pub struct GreekSyllablesIter<'s> {
    syllables: &'s GreekSyllables<'s>,
    head: usize,
    tail: isize,
}

impl<'s> GreekSyllablesIter<'s> {
    pub fn new(syllables: &'s GreekSyllables<'s>) -> Self {
        Self {
            syllables,
            head: 0,
            tail: syllables.len() as isize - 1,
        }
    }
}

impl<'s> Iterator for GreekSyllablesIter<'s> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head <= self.tail as usize {
            let res = &self.syllables.word[self.syllables.syllable_ranges[self.head].clone()];
            self.head += 1;
            Some(res)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for GreekSyllablesIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.tail >= self.head as isize {
            let res =
                &self.syllables.word[self.syllables.syllable_ranges[self.tail as usize].clone()];
            self.tail -= 1;
            Some(res)
        } else {
            None
        }
    }
}

pub struct GreekSyllablesPositionsIter<'s> {
    syllables: &'s GreekSyllables<'s>,
    cursor: usize,
}

impl<'s> Iterator for GreekSyllablesPositionsIter<'s> {
    type Item = (Range<usize>, &'s str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.syllables.syllable_ranges.len() {
            let syl = &self.syllables.word[self.syllables.syllable_ranges[self.cursor].clone()];
            let pos = self.syllables.syllable_ranges[self.cursor].clone();
            self.cursor += 1;
            Some((pos, syl))
        } else {
            None
        }
    }
}

pub struct SyllableAccentInfo {
    pub syllable_index: usize,
    pub accent_count: u8,
}

pub struct GreekSyllables<'s> {
    pub word: &'s str,
    syllable_ranges: Vec<Range<usize>>,
}

impl Index<usize> for GreekSyllables<'_> {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        &self.word[self.syllable_ranges[index].clone()]
    }
}

impl<'s> GreekSyllables<'s> {
    pub fn new(word: &'s str, norm_word: &str) -> Self {
        fn advance_to_next_matching_char(iter: &mut Peekable<CharIndices>, c: char) -> usize {
            let mut res = 0;
            while let Some((curr_word_i, curr_word_c)) = iter.peek() {
                if curr_word_c.to_lowercase().nfd().any(|wc| wc == c) {
                    res = *curr_word_i;
                    break;
                }
                iter.next();
            }
            res
        }

        let mut syllable_ranges = Vec::new();
        let mut norm_iter = norm_word.chars().peekable();
        let mut word_iter = word.char_indices().peekable();

        while let Some(norm_c) = norm_iter.next() {
            // If the character is not a vowel, just continue to the next
            if !norm_c.is_greek_vowel() {
                continue;
            }

            let word_i = advance_to_next_matching_char(&mut word_iter, norm_c);

            // We finished. No more characters
            let Some(next_norm_c) = norm_iter.peek().cloned() else {
                syllable_ranges.push(word_i..word.len());
                break;
            };

            let next_word_i =
                if next_norm_c.is_greek_vowel() && [norm_c, next_norm_c].is_diphthong() {
                    norm_iter.next();
                    word_iter.next();

                    let Some(second_next_norm_c) = norm_iter.peek().cloned() else {
                        syllable_ranges.push(word_i..word.len());
                        break;
                    };

                    advance_to_next_matching_char(&mut word_iter, second_next_norm_c)
                } else {
                    advance_to_next_matching_char(&mut word_iter, next_norm_c)
                };

            syllable_ranges.push(word_i..next_word_i);
        }

        Self {
            word,
            syllable_ranges,
        }
    }

    #[allow(clippy::len_without_is_empty)]
    #[inline]
    pub fn len(&self) -> usize {
        self.syllable_ranges.len()
    }

    pub fn iter_syllables(&'s self) -> GreekSyllablesIter<'s> {
        GreekSyllablesIter::new(self)
    }

    pub fn iter_syllables_with_positions(&'s self) -> GreekSyllablesPositionsIter<'s> {
        GreekSyllablesPositionsIter {
            syllables: self,
            cursor: 0,
        }
    }

    pub fn accent_info(&self) -> Option<SyllableAccentInfo> {
        let mut accent_count = 0;
        let mut syllable_index = 0;

        for (i, syllable) in self.iter_syllables().enumerate() {
            if syllable.chars().any(|c| c.is_greek_accented()) {
                accent_count += 1;
                syllable_index = i;
            }
        }

        if accent_count > 0 {
            SyllableAccentInfo {
                syllable_index,
                accent_count,
            }
            .into()
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn get_syllable_unchecked(&self, index: usize) -> &str {
        &self[index]
    }

    pub fn get_range_unchecked(&self, index: usize) -> Range<usize> {
        self.syllable_ranges[index].clone()
    }
}
