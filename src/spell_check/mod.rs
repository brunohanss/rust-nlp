// use core::fmt::Error;
// use symspell::{AsciiStringStrategy, StringStrategy, SymSpell, SymSpellBuilder, Verbosity};

// pub fn load() -> SymSpell<AsciiStringStrategy> {
//     // let builder = SymSpellBuilder::build(&self);
//     // Ok(symspell);
//     let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();
//     // let builder: SymSpellBuilder = symspell::SymSpellBuilder::default().build();
//     // symspell::SymSpellBuilder::max_dictionary_edit_distance(&mut symspell::SymSpellBuilder , 40)
//     let dictionary_imported =
//         symspell.load_dictionary("dictionaries/frequency_dictionary_en_82_765.txt", 0, 1, " ");
//     symspell.load_bigram_dictionary(
//         "dictionaries/frequency_bigramdictionary_en_243_342.txt",
//         0,
//         2,
//         " ",
//     );

//     println!("Dictionary added correctly : {:?}", dictionary_imported);
//     let suggestions = symspell.lookup("roket", Verbosity::Top, 2);
//     println!("{:?}", suggestions);

//     let sentence = "whereis th elove hehad dated forImuch of thepast who couqdn'tread in sixtgrade and ins pired him";
//     let compound_suggestions = symspell.lookup_compound(sentence, 2);
//     println!("{:?}", compound_suggestions);
//     symspell
// }
// pub fn new() {
//     // let suggestions = symspell.lookup("roket", Verbosity::Top, 2);
//     // println!("{:?}", suggestions);

//     // let sentence = "whereis th elove hehad dated forImuch of thepast who couqdn'tread in sixtgrade and ins pired him";
//     // let compound_suggestions = symspell.lookup_compound(sentence, 2);
//     // println!("{:?}", compound_suggestions);
// }
