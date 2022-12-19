SHELL := /bin/bash

.PHONY:
all: doc clean move-target-doc

.PHONY:
ja-readme:
	(( awk '{ if (NR == 1) { print "<a id=\"english\">\n" }; if (NR == 2) { print "[日本語版を読む](#japanese)" }; print $0;}' < README\ \(English\).md ) && ( awk '{ if (NR == 1) { print "<a id=\"japanese\">\n" }; if (NR == 2) { print "[Read in English](#english)" } print $0;}' < README（日本語版）.md )) > README.md
	(( awk '{ if (NR == 1) { print "<a id=\"english\">\n" }; if (NR == 2) { print "[日本語版を読む](#japanese)" }; print $0;}' < GPT-3\ \(English\).md ) && ( awk '{ if (NR == 1) { print "<a id=\"japanese\">\n" }; if (NR == 2) { print "[Read in English](#english)" } print $0;}' < GPT-3（日本語版）.md )) > GPT-3.md

.PHONY:
doc:
	cargo doc --no-deps

.PHONY:
clean:
	find . -mindepth 1 -maxdepth 1 -not -name 'target' -and -not -name '.git' -and -not -name '.gitignore' -and -not -name 'Makefile' | parallel rm -r

.PHONY:
move-target-doc:
	find target/doc -mindepth 1 -maxdepth 1 -not -name '.lock' | parallel --bar mv {} .
	find . -maxdepth 1 -mindepth 1 -not -name 'Makefile' -and -not -name '.git' -and -not -name '.lock' -and -not -name 'target' | parallel --jobs 1 git add
