all:
	(( awk '{ if (NR == 1) { print "<a id=\"english\">\n" }; if (NR == 2) { print "[日本語版を読む](#japanese)" }; print $0;}' < README\ \(English\).md ) && ( awk '{ if (NR == 1) { print "<a id=\"japanese\">\n" }; if (NR == 2) { print "[Read in English](#english)" } print $0;}' < README（日本語版）.md )) > README.md
	(( awk '{ if (NR == 1) { print "<a id=\"english\">\n" }; if (NR == 2) { print "[日本語版を読む](#japanese)" }; print $0;}' < GPT-3\ \(English\).md ) && ( awk '{ if (NR == 1) { print "<a id=\"japanese\">\n" }; if (NR == 2) { print "[Read in English](#english)" } print $0;}' < GPT-3（日本語版）.md )) > GPT-3.md
