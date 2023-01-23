use lingua::Language::{English, French};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

pub fn load() -> LanguageDetector {
    let languages = vec![English, French];
    let detector = LanguageDetectorBuilder::from_languages(&languages).build();
    detector
}
pub fn detect(detector: LanguageDetector, input: &str) {
    let detected_language: Option<Language> = detector.detect_language_of(input);
    match detected_language {
        Some(English) => println!("This is english"),
        Some(French) => println!("This is french"),
        None => println!("No language or error"),
    }
}
