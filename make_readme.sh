#!/bin/bash



table="| Day | Language | Execuction time | Comment |\n"
table+="|:---:|:---:| ---:| --- |\n"
for day in $(seq -f "%02g" 1 25); do
    [[ ! -d "$day" ]] && continue
    # echo $day
    lang=$(grep -E "Today's language: "'\*\*[a-zA-Z0-9_]+\*\*' $day/README.md | sed -r 's/.*\*\*(.+)\*\*.*/\1/')
    exe=$(find $day -type f -executable)
    if [[ $(echo $exe | wc -w) != 1 ]]; then
        echo "Found multiple or no executables for day $day: '$exe'"
        time=Error
    else
        time=$({ time ./$exe; } 2>&1 | tail -3 | head -1 | awk '{print $2}')
        re='([[:digit:]]+)m([[:digit:]]+),([[:digit:]]+)s'
        [[ $time =~ $re ]]
        min=${BASH_REMATCH[1]}
        sec=${BASH_REMATCH[2]}
        msec=${BASH_REMATCH[3]}
        if [[ $min != 0 ]]; then
            time="$min m $sec,$msec s"
        else
            time="$sec,$msec s"
        fi
        # elif [[ $sec != 0 ]]; then
        #     time="$sec,$msec s"
        # else
        #     time="$((msec+0)) ms"
        # fi
    fi
    comment=$(grep -E "<!-- .* -->" $day/README.md | head -1 | sed -r 's/.*<!-- *(.*) *-->.*/\1/')
    table+="| [$day]($day) | $lang | $time | $comment |\n"
    
done
# remove table from readme
sed -i '/<!-- table begin -->/q' README.md
echo -e $table | column -t -s '|' -o '|' >> README.md
