#!/bin/bash

# Die YAML-Datei mit den Fragen
input_file="LPCI1+2_100Questions.yaml"
output_file="fragen_mit_hash.yaml"

# Das Skript erstellt eine neue Datei, in der die Hashes hinzugefügt werden
> "$output_file"

# Funktion zum Berechnen des MD5-Hashes
generate_md5_hash() {
    echo -n "$1" | md5sum | awk '{ print $1 }'
}

# Lese jede Zeile der YAML-Datei
while IFS= read -r line; do
    # Wenn eine Frage gefunden wird
    if [[ "$line" =~ question: ]]; then
        # Extrahiere den Frage-Text
        question_text=$(echo "$line" | sed 's/question: "//' | sed 's/"$//')

        # Berechne den MD5-Hash der Frage
        hash=$(generate_md5_hash "$question_text")

        # Füge die Frage und den MD5-Hash als neuen Schlüssel ein
        echo "$line" >> "$output_file"
        echo "    hash: \"$hash\"" >> "$output_file"
    else
        # Andere Zeilen werden einfach in die Ausgabedatei übernommen
        echo "$line" >> "$output_file"
    fi
done < "$input_file"

echo "Die YAML-Datei mit den Hashes wurde als '$output_file' gespeichert."
