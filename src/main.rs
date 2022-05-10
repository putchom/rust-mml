mod mml_parser;
mod wav_writer;

fn main() {
    let mml = format!(
        "{}{}{}{}{}",
        "o6 l4 dl2dl4e do5bo6d r dl2dl4e do5bo6d r",
        "o5 l4 gggal2bl4bl4g l2bl4bl4o6dl2d r",
        "o5 l4 gggal2bl4b r gggal2bl4b r",
        "o5 l4 aaaga r b r o6d r c r o5b r a r",
        "o6 l4 dl2dl4e do5bo6d r dl2dl4e do5ba r l2gg"
    );
    let notes = mml_parser::parse(mml);
    let bpm = 240.0;
    wav_writer::write("yobikomi.wav", notes, bpm);
}
