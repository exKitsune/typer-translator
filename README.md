# typer-translator

This project aims to haphazardly assist in learning a new language by allowing a person to learn their most commonly used words in everyday speech first.
It is meant to be used in conjunction with other learning tools such as flashcards or an actual study plan.

When you launch this program, it should open a webpage in your browser, and by being in your browser, you can install a plugin such as [yomichan](https://github.com/FooSoft/yomichan) which allows you to instantly translate words using user provided dictionaries as well as [anki](https://itazuraneko.neocities.org/learn/anki.html) integration.

You can create a free DeepL account here: https://www.deepl.com/en/docs-api/

## Requirements

Python installation version > 3.8.2

# How this works:

## 0. Download latest release from [here](https://github.com/exKitsune/typer-translator/tags). Run the exe and head to http://localhost:35465/
   * Place the exe in a folder, and on first run, a database and config file will spawn.
     Please choose a language to translate to using https://www.sitepoint.com/iso-2-letter-language-codes/ and then choose the translation service and add your API key.
   * Upon running the exe, it will show up in your taskbar tray, you can left click to go to the site, and right click to easily access config.
## 1. Records your keystrokes 
   * You can set which windows it will be active for, so you don't record your logins on browser for example, but only for discord
     * There exists process selection and window title selection
     * Process selection is nice for applications like discord
     * Window title selection is nice for logging things like your browser but only specific tabs. 
     * One thing to note though, titles tend to change a lot, meaning if a discord server changes its name, you'll have to whitelist something new.
     Browser tabs might also change their title frequently so please look out for that if you opt to choose window title based whitelisting.
     To stay on top of things, I have included a last 4 processes/windows accessed ticker that helps you know what you have been writing in.
## 2. Parses your keystrokes to form sentences
   * Things to note:
     * Some keyboard shortcuts won't be logged
     * Copy/Paste will not be logged
     * Moving your cursor with mouse/arrows will not be logged
## 3. Sends your sentence through a translator (I've only tested DeepL free)
   * We use the translator package, it offers: 
     * Microsoft Translation API
     * Translated MyMemory API
     * LibreTranslate
     * DeepL’s free and pro APIs
     * https://translate-python.readthedocs.io/en/latest/providers.html
     * https://github.com/terryyin/translate-python/blob/master/translate/translate.py
## 4. Displays your recent message next to its translation
   * Right click to save or delete this message and its translation!
## 5. Save your favorite phrases
   * You can use yomichan + Anki integration to make a flashcard!
  

![typer-translator demo](https://github.com/exKitsune/typer-translator/blob/master/demo.gif)

## If you'd like to edit/build yourself

You need python and npm installed on your machine

Simply `pip install requirements.txt` in the base folder (make yourself a virtual environment first) and `npm install` in the client folder. To build the client, you can then run `npm run build` in the client folder, this will generate the actual website.

You can simply run the `main.py` file for testing

When you're ready to build, run `pyinstaller typer-translator.spec` and then check in the newly created `dist` folder for the executable