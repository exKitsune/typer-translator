# typer-translator

This project aims to haphazardly assist in learning a new language by allowing a person to learn their most commonly used words in everyday speech first.
It is meant to be used in conjunction with other learning tools such as flashcards or an actual study plan.

When you launch this program, it should open a webpage in your browser, and by being in your browser, you can install a plugin such as [yomichan](https://github.com/FooSoft/yomichan) which allows you to instantly translate words using user provided dictionaries as well as anki integration.

You can create a free DeepL account here: https://www.deepl.com/en/docs-api/

How this works:
1. Records your keystrokes 
   * You can set which windows it will be active for, so you don't record your logins on browser for example, but only for discord
     * There exists process selection and window title selection
     * Process selection is nice for applications like discord
     * Window title selection is nice for logging things like your browser but only specific tabs. 
     * One thing to note though, titles tend to change a lot, meaning if a discord server changes its name, you'll have to whitelist something new.
     Browser tabs might also change their title frequently so please look out for that if you opt to choose window title based whitelisting.
     To stay on top of things, I have included a last 4 processes/windows accessed ticker that helps you know what you have been writing in.
2. Parses your keystrokes to form sentences
3. Sends your sentence through a translator (I've only tested DeepL free)
   * We use the translator package, it offers: 
     * Microsoft Translation API
     * Translated MyMemory API
     * LibreTranslate
     * DeepL’s free and pro APIs
     * https://www.sitepoint.com/iso-2-letter-language-codes/
     * https://translate-python.readthedocs.io/en/latest/providers.html
     * https://github.com/terryyin/translate-python/blob/master/translate/translate.py
4. Displays your recent message next to its translation
5. Save your favorite phrases

## If you'd like to edit/build yourself

You need python and npm installed on your machine

Simply `pip install requirements.txt` in the base folder and `npm install` in the client folder

You can simply run the `main.py` file for testing

When you're ready to build, run `pyinstaller typer-translator.spec` and then check in the newly created `dist` folder for the executable