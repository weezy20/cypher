# Cypher:
## Implementation of Caesar and Vigenère ciphers

- A Caesar cipher takes a numerical value between 1-25 inclusive and shifts each character by that amount and returns an encrypted message. You can decrypt the same message by unshifting
by the same amount only if you know the shift value. Even if you don't, you can try all 25 combinations of shift values till you get a message that looks like English.

- A Vigenère cipher is an improved version of the Caesar cipher, where the shift values are not constant but derived from the position values of letters of a secret key in the alphabet.
This makes it more difficult computationally to crack a Vigenère cipher if you don't know what keys were used. But you can still crack it nonetheless. If you know the key's length, you 
can brute force this approach. If you don't you can look at the cipher text and look for repeating patterns, as usually English messages have a lot of repeating words. If you find
a pattern such as `WQL` twice in a sentence, you can estimate that the key's length might be 3. Another step is frequency analysis where you can exploit the uneven distribution of letters
in the language. The letter E is the most common letter in the English language so if your cipher text has a lot of X's, chances are that X corresponds to E. Using a combination of these techniques
and enough computational power, one may easily crack a Vigenère cipher.
