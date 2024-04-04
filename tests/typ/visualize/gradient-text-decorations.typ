// Tests gradients on text decorations.

--- gradient-linear-text-decoration ---
#set text(fill: gradient.linear(red, blue))

Hello #underline[World]! \
Hello #overline[World]! \
Hello #strike[World]! \
