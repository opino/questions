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

Einfacher Gewichtungsansatz
- Jede Frage hat ein Gewicht (z. B. ein Zahlenwert zwischen 1 und 10).
-- Startgewicht = 5
--- Richtige Antwort: Gewicht −1 (bis Minimum 1)
--- Falsche Antwort: Gewicht +2 (bis Maximum 10)

