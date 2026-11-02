//! Text attributes and the [`Effects`] bitset that stores them.

use std::fmt;

/// A text attribute (SGR parameter) such as bold or underline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Attribute {
    /// Bold or increased intensity.
    Bold,
    /// Dim / decreased intensity.
    Dim,
    /// Italic.
    Italic,
    /// Underlined.
    Underline,
    /// Slow blink.
    Blink,
    /// Reverse video (swap foreground and background).
    Reverse,
    /// Hidden (concealed) text.
    Hidden,
    /// Strikethrough.
    Strikethrough,
}

impl Attribute {
    /// All attributes, in canonical emission order.
    pub const ALL: [Attribute; 8] = [
        Self::Bold,
        Self::Dim,
        Self::Italic,
        Self::Underline,
        Self::Blink,
        Self::Reverse,
        Self::Hidden,
        Self::Strikethrough,
    ];

    /// Returns the raw SGR parameter for this attribute.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Bold => "1",
            Self::Dim => "2",
            Self::Italic => "3",
            Self::Underline => "4",
            Self::Blink => "5",
            Self::Reverse => "7",
            Self::Hidden => "8",
            Self::Strikethrough => "9",
        }
    }

    /// The bit this attribute occupies inside [`Effects`].
    #[must_use]
    pub const fn bit(self) -> u16 {
        1 << self.bit_index()
    }

    /// Position of this attribute's bit inside [`Effects`].
    #[must_use]
    pub const fn bit_index(self) -> u16 {
        match self {
            Self::Bold => 0,
            Self::Dim => 1,
            Self::Italic => 2,
            Self::Underline => 3,
            Self::Blink => 4,
            Self::Reverse => 5,
            Self::Hidden => 6,
            Self::Strikethrough => 7,
        }
    }
}

/// A compact, `Copy` bitset of text attributes.
///
/// [`Effects`] replaces heap-allocated attribute lists: adding an attribute is
/// a single `or` operation, duplicates are impossible, and the whole set is
/// usable in `const` contexts.
///
/// # Examples
///
/// ```
/// use pretty_console::{Attribute, Effects};
///
/// let effects = Effects::BOLD.union(Attribute::Italic.into());
/// assert!(effects.has(Attribute::Bold));
/// assert!(effects.contains(Effects::ITALIC));
/// assert_eq!(effects.attributes().count(), 2);
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Effects(u16);

impl Effects {
    /// No attributes set.
    pub const EMPTY: Self = Self(0);
    /// Bold.
    pub const BOLD: Self = Self(1 << 0);
    /// Dim.
    pub const DIM: Self = Self(1 << 1);
    /// Italic.
    pub const ITALIC: Self = Self(1 << 2);
    /// Underline.
    pub const UNDERLINE: Self = Self(1 << 3);
    /// Blink.
    pub const BLINK: Self = Self(1 << 4);
    /// Reverse video.
    pub const REVERSE: Self = Self(1 << 5);
    /// Hidden text.
    pub const HIDDEN: Self = Self(1 << 6);
    /// Strikethrough.
    pub const STRIKETHROUGH: Self = Self(1 << 7);
    /// Every known attribute.
    pub const ALL: Self = Self(0x00FF);

    /// Builds a set from raw bits; unknown bits are masked off.
    #[must_use]
    pub const fn from_bits(bits: u16) -> Self {
        Self(bits & 0x00FF)
    }

    /// The raw bits of this set.
    #[must_use]
    pub const fn bits(self) -> u16 {
        self.0
    }

    /// Returns `true` when no attribute is set.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` when every attribute in `other` is also in `self`.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Returns `true` when `attribute` is set.
    #[must_use]
    pub const fn has(self, attribute: Attribute) -> bool {
        self.0 & attribute.bit() != 0
    }

    /// Adds `attribute`.
    #[must_use]
    pub const fn insert(self, attribute: Attribute) -> Self {
        Self(self.0 | attribute.bit())
    }

    /// Removes `attribute`.
    #[must_use]
    pub const fn remove(self, attribute: Attribute) -> Self {
        Self(self.0 & !attribute.bit())
    }

    /// Union of both sets.
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Every attribute of `self` that is not in `other`.
    #[must_use]
    pub const fn without(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// Iterates the contained attributes in canonical emission order.
    #[must_use]
    pub fn attributes(self) -> EffectIter {
        EffectIter { bits: self.0 }
    }
}

impl From<Attribute> for Effects {
    fn from(attribute: Attribute) -> Self {
        Self::EMPTY.insert(attribute)
    }
}

impl std::ops::BitOr for Effects {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        self.union(rhs)
    }
}

impl std::ops::BitOrAssign for Effects {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::Sub for Effects {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.without(rhs)
    }
}

impl fmt::Display for Effects {
    /// Prints the SGR parameters of the set, e.g. `1;4` for bold + underline.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for attr in self.attributes() {
            if !first {
                f.write_str(";")?;
            }
            first = false;
            f.write_str(attr.code())?;
        }
        Ok(())
    }
}

/// Iterator over the attributes contained in an [`Effects`] set.
///
/// Produced by [`Effects::attributes`].
#[derive(Debug, Clone, Copy)]
pub struct EffectIter {
    bits: u16,
}

// A Copy iterator over a Copy element is intentional: it is a cursor over a
// u16 bitset, cheaper to copy than to borrow.
#[allow(clippy::copy_iterator)]
impl Iterator for EffectIter {
    type Item = Attribute;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bits == 0 {
            return None;
        }
        let index = self.bits.trailing_zeros() as u16;
        self.bits &= self.bits - 1;
        let attribute = Attribute::ALL[index as usize];
        Some(attribute)
    }
}

impl ExactSizeIterator for EffectIter {
    fn len(&self) -> usize {
        self.bits.count_ones() as usize
    }
}
