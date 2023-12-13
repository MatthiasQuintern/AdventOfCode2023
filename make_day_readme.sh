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
    # args: 1: fileext, 2: lang, 3: howto
    sources=($(IFS=$'\n' ls $day_dir/*.$1 2> /dev/null))
    [[ $? != 0 ]] && return
    if [[ ${#sources[@]} -gt 1 ]]; then
        printf "$FMT_WARN" "Found multiple sources for language $2 (using first as main): ${sources[@]}"
    fi
    if [[ ${#sources[@]} -gt 1 ]]; then
        printf "$FMT_WARN" "Found multiple sources for language $2 (using first as main): ${sources[@]}"
    fi
    mainsource=${sources[0]}
    if [[ -z $lang ]]; then
        lang=$2
    else
        lang=$lang,$2
    fi
    mainsource_base=$(basename $mainsource)
    printf "$FMT_VAR" "Determined Language" "$2 ($mainsource_base)"
    if [[ -n $3 ]]; then
        howto=$(echo "$3" | sed "s|SOURCE|$mainsource_base|")
    else
        printf "$FMT_WARN" "No HOWTO provided for language $2"
    fi
    for file in ${sources[@]}; do
        loc=$(( $loc + $(sed -r '/^\s*(#|\/\/|\/\*|;)/d;/^\s*$/d' "$file" | wc -l) ))
    done
}

function get_time {
    cd $day_dir
    exe=$(find . -type f -executable)
    if [[ -z $exe ]]; then
        printf "$FMT_ERR" "Found no executable for day $day"
        cd ..
        return 2
    fi
    exe=$(basename $exe)
    if [[ $(echo $exe | wc -w) != 1 ]]; then
        printf "$FMT_ERR" "Found multiple executables for day $day: '$exe'"
        cd ..
        return 1
    fi
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
    # elif [[ $sec != 0 ]]; then
    #     time="$sec,$msec s"
    # else
    #     time="$((msec+0)) ms"
    # fi
    cd ..
}



gifts=""
for i in $(seq $day); do gifts+=":gift:"; done
sed "s/DAY/$day/g" README.md.temp | sed "s/:gift:/$gifts/" > "$readme"
get_time
if [[ $? == 0 ]]; then
    printf "$FMT_VAR" "exectime" "$time"
    sed -i "s/EXECTIME/$time/" "$readme"
else
    printf "$FMT_WARN" "No execution time determined"
    sed -i '/.*EXECTIME.*/d' "$readme"
fi

if [[ -z $1 ]]; then
    printf "$FMT_WARN" "No comment provided"
    sed -i '/<!-- COMMENT -->/d' "$readme" || echo "ERROR comment"
else
    printf "$FMT_VAR" "Comment" "'$1'"
    sed -i "s|COMMENT|$1|" "$readme" || echo "ERROR comment2"
fi

dayexec="day${day_dir}"
loc=0
find_lang py Python "python3 SOURCE"
find_lang rs Rust "rustc SOURCE\n./$dayexec"
find_lang cpp C++ "g++ SOURCE -o ${dayexec}\n./${dayexec}"
find_lang c C "gcc SOURCE -o ${dayexec}\n./${dayexec}"
find_lang java Java "javac Main.java\njava Main"
find_lang s Assembly ""
find_lang s65 Assembly "javac Main.java\njava Main"
find_lang awk Awk "awk -f SOURCE input.txt"
find_lang sh Bash "./SOURCE"
find_lang php PhP "php -S localhost:8000 -t .\nfirefox http://localhost:8000"
find_lang js Javascript ""

if [[ -z $lang ]]; then
    printf "$FMT_WARN" "No languages found"
else
    sed -i "s/LANG/$lang/" $readme
fi

if [[ -n $howto ]]; then
    sed -i "s|HOWTO|$howto|" $readme
fi

if [[ $loc == 0 ]]; then
    printf "$FMT_WARN" "No lines of code found"
else
    sed -i "s(LOC($loc(" $readme
fi
