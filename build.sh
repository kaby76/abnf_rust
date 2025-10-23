# Generated from trgen 0.23.21
set -e
if [ -f transformGrammar.py ]; then python3 transformGrammar.py ; fi

JAR='c:/Users/Kenne/Downloads/antlr4-4.13.3-SNAPSHOT-complete.jar'
java -jar $JAR -encoding utf-8 -Dlanguage=Rust -o src/abnf Abnf.g4

cargo b

exit 0
