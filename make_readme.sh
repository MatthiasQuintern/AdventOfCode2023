#!/bin/bash

table="| Day | Language | Lines of code | Execuction time | Comment |\n"
table+="|:---:|:---:| ---:| ---: |--- |\n"
maxday=$(ls -d */ | sed -r 's|0*(.*)/|\1|g' | tail -1)
for day in $(seq -f "%02g" 1 $maxday); do
    if [[ ! -d "$day" ]]; then 
        echo "Missing directory for day $day"
        continue
    fi
    if [[ ! -f "$day/README.md" ]]; then 
        echo "Missing README for day $day"
        continue
    fi
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
