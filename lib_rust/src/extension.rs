use crate::CLITICS;
use std::ops::{Bound, RangeBounds};
use unic_ucd_category::GeneralCategory;
use unic_ucd_normal::CanonicalCombiningClass;
use unicode_normalization::UnicodeNormalization;

pub trait NormalizationExtensions: ToOwned {
    fn normalize_greek(&self) -> <Self as ToOwned>::Owned;
}

impl NormalizationExtensions for str {
    fn normalize_greek(&self) -> <Self as ToOwned>::Owned {
        self.to_lowercase()
            .nfd()
            .filter(|c| CanonicalCombiningClass::of(*c).number() == 0)
            .collect()
    }
}

pub trait DiphthongExtensions {
    fn is_diphthong(&self) -> bool;
}

impl DiphthongExtensions for str {
    fn is_diphthong(&self) -> bool {
        if self.chars().count() != 2 {
            return false;
        }

        let mut chars_iter = self.chars();
        let chars = [chars_iter.next().unwrap(), chars_iter.next().unwrap()];
        chars.is_diphthong()
    }
}
impl DiphthongExtensions for [char; 2] {
    fn is_diphthong(&self) -> bool {
        matches!(
            self,
            ['α', 'ι']
                | ['ε', 'ι']
                | ['ο', 'ι']
                | ['α', 'υ']
                | ['ε', 'υ']
                | ['η', 'υ']
                | ['ο', 'υ']
        )
    }
}

pub trait VowelExtensions
where
    Self: Copy,
{
    fn is_greek_vowel(self) -> bool;

    fn is_greek_accented(self) -> bool;

    fn is_short_vowel(self) -> bool;

    fn is_long_vowel(self) -> bool;

    fn is_mutable_vowel(self) -> bool;

    fn to_acute(self) -> Self;

    fn to_circumflex(self) -> Self;

    fn to_grave(self) -> Self;
}

impl VowelExtensions for char {
    fn is_greek_vowel(self) -> bool {
        matches!(
            self,
            'α' | 'Α' | 'ε' | 'Ε' | 'η' | 'Η' | 'ι' | 'Ι' | 'ο' | 'Ο' | 'υ' | 'Υ' | 'ω' | 'Ω'
        )
    }

    fn is_greek_accented(self) -> bool {
        matches!(
            self,
            'ά' | 'Ά' | 'έ' | 'Έ' | 'ή' | 'Ή' | 'ί' | 'Ί' | 'ό' | 'Ό' | 'ύ' | 'Ύ' | 'ώ' | 'Ώ'
        )
    }

    fn is_short_vowel(self) -> bool {
        matches!(self, 'ε' | 'ο' | 'Ε' | 'Ο')
    }

    fn is_long_vowel(self) -> bool {
        matches!(self, 'η' | 'ω' | 'Η' | 'Ω')
    }

    fn is_mutable_vowel(self) -> bool {
        matches!(self, 'α' | 'ι' | 'υ' | 'Α' | 'Ι' | 'Υ')
    }

    fn to_acute(self) -> Self {
        match self {
            'α' => 'ά',
            'Α' => 'Ά',
            'η' => 'ή',
            'Η' => 'Ή',
            'ω' => 'ώ',
            'Ω' => 'Ώ',
            'ι' => 'ί',
            'Ι' => 'Ί',
            'υ' => 'ύ',
            'Υ' => 'Ύ',
            'ε' => 'έ',
            'Ε' => 'Έ',
            'ο' => 'ό',
            'Ο' => 'Ό',
            _ => self,
        }
    }

    fn to_circumflex(self) -> Self {
        match self {
            'ά' => 'ᾶ',
            'ή' => 'ῆ',
            'ώ' => 'ῶ',
            'ί' => 'ῖ',
            'ύ' => 'ῦ',
            _ => self,
        }
    }

    fn to_grave(self) -> Self {
        match self {
            'ά' => 'ὰ',
            'ή' => 'ὴ',
            'ώ' => 'ὼ',
            'ί' => 'ὶ',
            'ύ' => 'ὺ',
            'έ' => 'ὲ',
            'ό' => 'ὸ',
            _ => self,
        }
    }
}

pub trait AccentExtensions {
    fn make_acute_range<R: RangeBounds<usize>>(&mut self, range: R);

    #[allow(dead_code)]
    fn make_acute(&mut self) {
        self.make_acute_range(..);
    }

    fn make_circumflexed_range<R: RangeBounds<usize>>(&mut self, range: R);

    fn make_circumflexed(&mut self) {
        self.make_circumflexed_range(..);
    }

    fn make_graved_range<R: RangeBounds<usize>>(&mut self, range: R);

    fn make_graved(&mut self) {
        self.make_graved_range(..);
    }
}

fn get_string_from_range_bounds<'s, R: RangeBounds<usize>>(s: &'s str, bounds: &R) -> &'s str {
    match (bounds.start_bound(), bounds.end_bound()) {
        (Bound::Unbounded, Bound::Unbounded) => s,
        (Bound::Unbounded, Bound::Included(n)) => &s[..=*n],
        (Bound::Unbounded, Bound::Excluded(n)) => &s[..*n],
        (Bound::Included(n), Bound::Unbounded) => &s[*n..],
        (Bound::Included(start), Bound::Included(end)) => &s[*start..=*end],
        (Bound::Included(start), Bound::Excluded(end)) => &s[*start..*end],
        (Bound::Excluded(n), Bound::Unbounded) => &s[*n + 1..],
        (Bound::Excluded(start), Bound::Included(end)) => &s[*start + 1..=*end],
        (Bound::Excluded(start), Bound::Excluded(end)) => &s[*start + 1..*end],
    }
}

impl AccentExtensions for String {
    fn make_acute_range<R: RangeBounds<usize>>(&mut self, range: R) {
        let replacement = get_string_from_range_bounds(self, &range)
            .chars()
            .map(|c| c.to_acute())
            .collect::<String>();

        self.replace_range(range, &replacement);
    }

    fn make_circumflexed_range<R: RangeBounds<usize>>(&mut self, range: R) {
        let replacement = get_string_from_range_bounds(self, &range)
            .chars()
            .map(|c| c.to_circumflex())
            .collect::<String>();

        self.replace_range(range, &replacement);
    }

    fn make_graved_range<R: RangeBounds<usize>>(&mut self, range: R) {
        let replacement = get_string_from_range_bounds(self, &range)
            .chars()
            .map(|c| c.to_grave())
            .collect::<String>();

        self.replace_range(range, &replacement);
    }
}

pub trait GrammarExtensions {
    fn is_clitic(&self) -> bool;

    fn is_punctuation(&self) -> bool;
}

impl GrammarExtensions for str {
    fn is_clitic(&self) -> bool {
        CLITICS.contains(self.to_lowercase().as_str())
    }

    fn is_punctuation(&self) -> bool {
        self.chars()
            .all(|c| GeneralCategory::of(c).is_punctuation())
    }
}
