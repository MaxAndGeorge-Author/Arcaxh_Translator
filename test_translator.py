from arcaxh_translator import ArcaxhTranslator

def main():
    translator = ArcaxhTranslator()
    
    # Test some translations
    test_words = [
        "arkashir",
        "velkharn",
        "zorakhion"
    ]
    
    print("Testing Arcaxh Translator:")
    print("-----------------------")
    
    for word in test_words:
        translation = translator.translate_to_english(word)
        analysis = translator.analyze_word(word)
        print(f"\nWord: {word}")
        print(f"Translation: {translation}")
        print(f"Analysis: {analysis}")

if __name__ == "__main__":
    main()
