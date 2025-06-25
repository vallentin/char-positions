use char_positions::{CharPositionsExt, LineCol, LineColByteRange};

fn main() {
    let text = "Hello 👋\nWorld 🌏\n🦀🦀";

    for (LineCol(line, col), c) in text.char_positions() {
        println!("[Ln {line}, Col {col}] {c:?}");
    }

    let mut iter = text
        .char_positions::<LineColByteRange>()
        .map(|(LineColByteRange(line, col, range), c)| (line, col, range, c));

    assert_eq!(iter.next(), Some((1, 1, 0..1, 'H')));
    assert_eq!(iter.next(), Some((1, 2, 1..2, 'e')));
    assert_eq!(iter.next(), Some((1, 3, 2..3, 'l')));
    assert_eq!(iter.next(), Some((1, 4, 3..4, 'l')));
    assert_eq!(iter.next(), Some((1, 5, 4..5, 'o')));
    assert_eq!(iter.next(), Some((1, 6, 5..6, ' ')));
    assert_eq!(iter.next(), Some((1, 7, 6..10, '👋')));
    assert_eq!(iter.next(), Some((1, 8, 10..11, '\n')));
    assert_eq!(iter.next(), Some((2, 1, 11..12, 'W')));
    assert_eq!(iter.next(), Some((2, 2, 12..13, 'o')));
    assert_eq!(iter.next(), Some((2, 3, 13..14, 'r')));
    assert_eq!(iter.next(), Some((2, 4, 14..15, 'l')));
    assert_eq!(iter.next(), Some((2, 5, 15..16, 'd')));
    assert_eq!(iter.next(), Some((2, 6, 16..17, ' ')));
    assert_eq!(iter.next(), Some((2, 7, 17..21, '🌏')));
    assert_eq!(iter.next(), Some((2, 8, 21..22, '\n')));
    assert_eq!(iter.next(), Some((3, 1, 22..26, '🦀')));
    assert_eq!(iter.next(), Some((3, 2, 26..30, '🦀')));
    assert_eq!(iter.next(), None);
}
