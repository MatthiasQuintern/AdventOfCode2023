#!/bin/bash
# pass comment as param $1 and select day with day=X ./make_day_readme.sh
maxday=$(ls -d */ | sed -r 's|0*(.*)/|\1|g' | tail -1)
[[ -z $day ]] && day=$maxday
day_dir=$(printf "%02d" $day)
readme=$day_dir/README.md

FMT_VAR='\e[32m%s: \e[0m%s\n'
FMT_ERR='\e[31m%s\e[0m\n'
FMT_WARN='\e[33m%s\e[0m\n'

echo Making Readme of day $day in $readme

function find_lang {
    # 1: fileext, 2: lang, 3: howto
    sourcefile=$(ls $day_dir/*.$1 2> /dev/null)
    [[ $? != 0 ]] && return
    lang=$2
    sourcefilebase=$(basename $sourcefile)
    printf "$FMT_VAR" "Determined Langugae" "$2 ($sourcefilebase)"
    sed -i "s/LANG/$2/" $readme
    howto=$(echo "$3" | sed "s|SOURCE|$sourcefilebase|")
    sed -i "s(HOWTORUN($howto(" $readme
    loc=$(sed -r '/^\s*(#|\/\/|\/\*|;)/d;/^\s*$/d' $sourcefile | wc -l)
    sed -i "s(LOC($loc(" $readme
}

function get_time {
    cd $day_dir
    exe=$(find . -type f -executable)
    exe=$(basename $exe)
    if [[ $(echo $exe | wc -w) != 1 ]]; then
        printf "$FMT_ERR" "Found multiple or no executables for day $day: '$exe'"
        cd ..
        return 1
        time=Error
    else
        time=$({ time ./$exe; } 2>&1)
        if [[ $? != 0 ]]; then
            printf "$FMT_ERR" "Execution error in './$exe' in '$(pwd)'. Output:"
            printf "\e[34m"
            echo -e "$time"
            printf "\e[0m"
            cd ..
            return 1
        fi
        time=$(echo -e "$time" | tail -3 | head -1 | awk '{print $2}')
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
        cd ..
        # elif [[ $sec != 0 ]]; then
        #     time="$sec,$msec s"
        # else
        #     time="$((msec+0)) ms"
        # fi
    fi
}



gifts=""
for i in $(seq $day); do gifts+=":gift:"; done
sed "s/DAY/$day/g" README.md.temp | sed "s/:gift:/$gifts/" > $day_dir/README.md
get_time
if [[ $? == 0 ]]; then
    cd ..
    printf "$FMT_VAR" "exectime" "$time"
    sed -i "s/EXECTIME/$time/" $readme
else
    cd ..
    printf "$FMT_WARN" "No execution time determined"
    sed -i '/.*EXECTIME.*/d' $readme
fi

if [[ -z $1 ]]; then
    printf "$FMT_WARN" "No comment provided"
    sed -i "/<!-- COMMENT -->/d" $readme
else
    printf "$FMT_VAR" "Comment" "'$1'"
    sed -i "s|COMMENT|$1|" $readme
fi

find_lang py Python "python3 SOURCE"
find_lang rs Rust "rustc SOURCE\n./day${day_dir}"
find_lang cpp C++ "g++ SOURCE -o day${day_dir}\n ./day${day_dir}"
