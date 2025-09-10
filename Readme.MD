## how to start:

cargo run

## Changelog:

### 05.09.2025

- Introducing "Why" tag, that holds the explanation  

### 04.09.2025 

- Fragen befinden sich nun im Ordner "Questions".
- Die YAML Dateien enthalten einen Beschreibungstag

## Featurerequest 001:

Die Fragen sollen bei falscher beantwortung in einen Stapel falsch.
-> jede Frage solle einen Hash erhalten, mit dem die Frage eindeutig indentifiziert werden kann.


## Featurerequest 002 Gewichtung:

1. Einfacher Gewichtungsansatz
- Jede Frage hat ein Gewicht (z. B. ein Zahlenwert zwischen 1 und 10).
  - Startgewicht = 5
    - Richtige Antwort: Gewicht −1 (bis Minimum 1)
    - Falsche Antwort: Gewicht +2 (bis Maximum 10)

2. Spaced Repetition Light (etwas klüger)
- Jede Frage bekommt:
  - einen Zähler für richtige Antworten
  - einen Zähler für falsche Antworten
  - ein "Nächstes Abfragedatum" (timestamp)

Falsche Antworten → nächstes Abfragedatum wird vorverlegt
Richtige Antworten → nächstes Abfragedatum wird nach hinten verschoben

So erscheinen schwierige Fragen öfter und früher.

3. Leitner-System (bekanntes Karteikartenprinzip)
- Fragen wandern durch "Boxen" (Levels).
  - Box 1: neue oder falsch beantwortete Fragen → täglich
  - Box 2: richtig beantwortet → alle 3 Tage
  - Box 3: richtig beantwortet → alle 7 Tage

Nach einer falschen Antwort → zurück in Box 1.

4. Gewichtung + Zufall kombiniert
- Wahrscheinlichkeit einer Frage = gewicht / summe aller gewichte
  - So wird nicht strikt nach Score ausgewählt, sondern nach Wahrscheinlichkeiten 
  – was Wiederholung garantiert, aber auch Abwechslung ermöglicht.

