// Test emoji shaping.

--- shaping-emoji-basic ---
// This should form a three-member family.
👩‍👩‍👦

// This should form a pride flag.
🏳️‍🌈

// Skin tone modifier should be applied.
👍🏿

// This should be a 1 in a box.
1️⃣

--- shaping-emoji-bad-zwj ---
// These two shouldn't be affected by a zero-width joiner.
🏞‍🌋
