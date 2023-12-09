#!/bin/bash

table="| Day | Language | Lines of code | Execuction time | Comment |\n"
table+="|:---:|:---:| ---:| --- |\n"
for day in $(seq -f "%02g" 1 25); do
    [[ ! -d "$day" ]] && continue
    # echo $day
    lang=$(grep -E "Today's language:" $day/README.md | sed -r 's/.*\*\*(.+)\*\*.*/\1/')
    exectime=$(grep -E "Execution time:" $day/README.md | sed -r 's/.*\*\*(.+)\*\*.*/\1/')
    loc=$(grep -E "Lines of code:" $day/README.md | sed -r 's/.*\*\*(.+)\*\*.*/\1/')
    comment=$(grep -E "<!-- .* -->" $day/README.md | head -1 | sed -r 's/.*<!-- *(.*) *-->.*/\1/')
    table+="| [$day]($day) | $lang | $loc | $exectime | $comment |\n"
done
# remove table from readme
sed -i '/<!-- table begin -->/q' README.md
echo -e $table | column -t -s '|' -o '|' >> README.md
echo -e "\n Lines of code are without blank lines and comments" >> README.md
