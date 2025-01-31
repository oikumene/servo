/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use ucd::{Codepoint, UnicodeBlock};

pub fn float_to_fixed(before: usize, f: f64) -> i32 {
    ((1i32 << before) as f64 * f) as i32
}

pub fn fixed_to_float(before: usize, f: i32) -> f64 {
    f as f64 * 1.0f64 / ((1i32 << before) as f64)
}

pub fn is_bidi_control(c: char) -> bool {
    matches!(c, '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}' | '\u{200E}' | '\u{200F}' | '\u{061C}')
}

pub fn unicode_plane(codepoint: char) -> u32 {
    (codepoint as u32) >> 16
}

pub fn is_cjk(codepoint: char) -> bool {
    if let Some(block) = codepoint.block() {
        match block {
            UnicodeBlock::CJKRadicalsSupplement |
            UnicodeBlock::KangxiRadicals |
            UnicodeBlock::IdeographicDescriptionCharacters |
            UnicodeBlock::CJKSymbolsandPunctuation |
            UnicodeBlock::Hiragana |
            UnicodeBlock::Katakana |
            UnicodeBlock::Bopomofo |
            UnicodeBlock::HangulCompatibilityJamo |
            UnicodeBlock::Kanbun |
            UnicodeBlock::BopomofoExtended |
            UnicodeBlock::CJKStrokes |
            UnicodeBlock::KatakanaPhoneticExtensions |
            UnicodeBlock::EnclosedCJKLettersandMonths |
            UnicodeBlock::CJKCompatibility |
            UnicodeBlock::CJKUnifiedIdeographsExtensionA |
            UnicodeBlock::YijingHexagramSymbols |
            UnicodeBlock::CJKUnifiedIdeographs |
            UnicodeBlock::CJKCompatibilityIdeographs |
            UnicodeBlock::CJKCompatibilityForms |
            UnicodeBlock::HalfwidthandFullwidthForms => return true,

            _ => {},
        }
    }

    // https://en.wikipedia.org/wiki/Plane_(Unicode)#Supplementary_Ideographic_Plane
    // https://en.wikipedia.org/wiki/Plane_(Unicode)#Tertiary_Ideographic_Plane
    unicode_plane(codepoint) == 2 || unicode_plane(codepoint) == 3
}
