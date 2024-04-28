# typer-translator

This project aims to haphazardly assist in learning a new language by allowing a person to learn their most commonly used words in everyday speech first.
It is meant to be used in conjunction with other learning tools such as flashcards or an actual study plan.

How this works:
1. Records your keystrokes 
   * You can set which windows it will be active for, so you don't record your logins on browser for example, but only for discord
2. Parses your keystrokes to form sentences
3. Sends your sentence through a translator
   * We use the translator package, it offers:
     * Microsoft Translation API
     * Translated MyMemory API
     * LibreTranslate
     * DeepL’s free and pro APIs
     * https://translate-python.readthedocs.io/en/latest/providers.html
4. Displays your recent message next to its translation
