// Test line breaks.

--- linebreak-overflow ---
// Test overlong word that is not directly after a hard break.
This is a spaceexceedinglylongy.

--- linebreak-overflow-double ---
// Test two overlong words in a row.
Supercalifragilisticexpialidocious Expialigoricmetrioxidation.

--- linebreak-hyphen-nbsp ---
// Test for non-breaking space and hyphen.
There are non\u{2011}breaking~characters.

--- linebreak-narrow-nbsp ---
// Test for narrow non-breaking space.
#show "_": sym.space.nobreak.narrow
0.1_g, 1_g, 10_g, 100_g, 1_000_g, 10_000_g, 100_000_g, 1_000_000_g

--- linebreak-shape-run ---
// Test that there are no unwanted line break opportunities on run change.
This is partly emp#emph[has]ized.

--- linebreak-manual ---
Hard #linebreak() break.

--- linebreak-manual-directly-after-automatic ---
// Test hard break directly after normal break.
Hard break directly after \ normal break.

--- linebreak-manual-consecutive ---
// Test consecutive breaks.
Two consecutive \ \ breaks and three \ \ more.

--- linebreak-manual-trailing-multiple ---
// Test forcing an empty trailing line.
Trailing break \ \

--- linebreak-manual-justified ---
// Test justified breaks.
#set par(justify: true)
With a soft #linebreak(justify: true)
break you can force a break without #linebreak(justify: true)
breaking justification. #linebreak(justify: false)
Nice!

--- comment-end-of-line ---
// Test comments at the end of a line
First part//
Second part

// Test comments at the end of a line with pre-spacing
First part          //
Second part

--- linebreak-thai ---
// Test linebreak for East Asian languages
ทีวีตรวจทานนอร์ทแฟรีเลคเชอร์โกลด์อัลบัมเชอร์รี่เย้วสโตร์กฤษณ์เคลมเยอบีร่าพ่อค้าบลูเบอร์รี่สหัสวรรษโฮปแคนูโยโย่จูนสตรอว์เบอร์รีซื่อบื้อเยนแบ็กโฮเป็นไงโดนัททอมสเตริโอแคนูวิทย์แดรี่โดนัทวิทย์แอปพริคอทเซอร์ไพรส์ไฮบริดกิฟท์อินเตอร์โซนเซอร์วิสเทียมทานโคโยตี้ม็อบเที่ยงคืนบุญคุณ
