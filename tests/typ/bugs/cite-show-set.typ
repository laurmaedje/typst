// Test show set rules on citations.

--- issue-2531-cite-show-set ---
#show cite: set text(red)
A @netwok @arrgh.
B #cite(<netwok>) #cite(<arrgh>).

#show bibliography: none
#bibliography("/assets/bib/works.bib")
