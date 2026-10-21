#![cfg(feature = "general-category")]

use unicode_properties::UnicodeGeneralCategory;

#[test]
fn general_category_test() {
    use std::ops::Not;
    use unicode_properties::UnicodeGeneralCategory;
    use unicode_properties::{GeneralCategory, GeneralCategoryGroup};
    assert_eq!('A'.general_category(), GeneralCategory::UppercaseLetter);
    assert_eq!('A'.general_category_group(), GeneralCategoryGroup::Letter);
    assert!('A'.is_letter_cased());
    assert_eq!(' '.general_category(), GeneralCategory::SpaceSeparator);
    assert_eq!(
        ' '.general_category_group(),
        GeneralCategoryGroup::Separator
    );
    assert!(' '.is_letter_cased().not());
    assert_eq!('一'.general_category(), GeneralCategory::OtherLetter);
    assert_eq!('一'.general_category_group(), GeneralCategoryGroup::Letter);
    assert!('一'.is_letter_cased().not());
    assert_eq!('🦀'.general_category(), GeneralCategory::OtherSymbol);
    assert_eq!('🦀'.general_category_group(), GeneralCategoryGroup::Symbol);
    assert!('🦀'.is_letter_cased().not());
}

#[test]
fn all_general_categories_to_group() {
    use unicode_properties::{GeneralCategory, GeneralCategoryGroup};
    assert_eq!(
        GeneralCategory::UppercaseLetter.general_category_group(),
        GeneralCategoryGroup::Letter
    );
    assert_eq!(
        GeneralCategory::LowercaseLetter.general_category_group(),
        GeneralCategoryGroup::Letter
    );
    assert_eq!(
        GeneralCategory::TitlecaseLetter.general_category_group(),
        GeneralCategoryGroup::Letter
    );
    assert_eq!(
        GeneralCategory::ModifierLetter.general_category_group(),
        GeneralCategoryGroup::Letter
    );
    assert_eq!(
        GeneralCategory::OtherLetter.general_category_group(),
        GeneralCategoryGroup::Letter
    );
    assert_eq!(
        GeneralCategory::NonspacingMark.general_category_group(),
        GeneralCategoryGroup::Mark
    );
    assert_eq!(
        GeneralCategory::SpacingMark.general_category_group(),
        GeneralCategoryGroup::Mark
    );
    assert_eq!(
        GeneralCategory::EnclosingMark.general_category_group(),
        GeneralCategoryGroup::Mark
    );
    assert_eq!(
        GeneralCategory::DecimalNumber.general_category_group(),
        GeneralCategoryGroup::Number
    );
    assert_eq!(
        GeneralCategory::LetterNumber.general_category_group(),
        GeneralCategoryGroup::Number
    );
    assert_eq!(
        GeneralCategory::OtherNumber.general_category_group(),
        GeneralCategoryGroup::Number
    );
    assert_eq!(
        GeneralCategory::ConnectorPunctuation.general_category_group(),
        GeneralCategoryGroup::Punctuation
    );
    assert_eq!(
        GeneralCategory::DashPunctuation.general_category_group(),
        GeneralCategoryGroup::Punctuation
    );
    assert_eq!(
        GeneralCategory::OpenPunctuation.general_category_group(),
        GeneralCategoryGroup::Punctuation
    );
    assert_eq!(
        GeneralCategory::ClosePunctuation.general_category_group(),
        GeneralCategoryGroup::Punctuation
    );
    assert_eq!(
        GeneralCategory::InitialPunctuation.general_category_group(),
        GeneralCategoryGroup::Punctuation
    );
    assert_eq!(
        GeneralCategory::FinalPunctuation.general_category_group(),
        GeneralCategoryGroup::Punctuation
    );
    assert_eq!(
        GeneralCategory::OtherPunctuation.general_category_group(),
        GeneralCategoryGroup::Punctuation
    );
    assert_eq!(
        GeneralCategory::MathSymbol.general_category_group(),
        GeneralCategoryGroup::Symbol
    );
    assert_eq!(
        GeneralCategory::CurrencySymbol.general_category_group(),
        GeneralCategoryGroup::Symbol
    );
    assert_eq!(
        GeneralCategory::ModifierSymbol.general_category_group(),
        GeneralCategoryGroup::Symbol
    );
    assert_eq!(
        GeneralCategory::OtherSymbol.general_category_group(),
        GeneralCategoryGroup::Symbol
    );
    assert_eq!(
        GeneralCategory::SpaceSeparator.general_category_group(),
        GeneralCategoryGroup::Separator
    );
    assert_eq!(
        GeneralCategory::LineSeparator.general_category_group(),
        GeneralCategoryGroup::Separator
    );
    assert_eq!(
        GeneralCategory::ParagraphSeparator.general_category_group(),
        GeneralCategoryGroup::Separator
    );
    assert_eq!(
        GeneralCategory::Control.general_category_group(),
        GeneralCategoryGroup::Other
    );
    assert_eq!(
        GeneralCategory::Format.general_category_group(),
        GeneralCategoryGroup::Other
    );
    assert_eq!(
        GeneralCategory::Surrogate.general_category_group(),
        GeneralCategoryGroup::Other
    );
    assert_eq!(
        GeneralCategory::PrivateUse.general_category_group(),
        GeneralCategoryGroup::Other
    );
    assert_eq!(
        GeneralCategory::Unassigned.general_category_group(),
        GeneralCategoryGroup::Other
    );
}
