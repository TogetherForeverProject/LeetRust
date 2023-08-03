#!/usr/bin/bash

# Function to extract the title from a problem file
# Arguments:
#   $1: The problem file path
# Returns:
#   The extracted title
function extract_title() {
    local problem_file=$1
    local title
    title=$(grep -Po "(?<=\/\/ Title: ).*" "$problem_file")
    echo "$title"
}

# Function to generate the main.rs content
# Arguments:
#   $1: The problem number
# Returns:
#   The generated main.rs content
function generate_content() {
    local problem_number=$1
    local content
    content=$(cat << EOF
use std::env;
mod problems;
use crate::problems::problem$problem_number;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <problem_number>", args[0]);
        return;
    }

    // Get the problem number from the first argument
    let problem_number: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid problem number provided.");
            return;
        }
    };

    // Call the appropriate problem's solve() function based on the problem_number
    match problem_number {
        $problem_number => problem$problem_number::solve(),
        _ => println!("Unknown problem number."),
    }
}
EOF
)
    echo "$content"
}

# Function to run a specific problem
# Arguments:
#   $1: The problem number
#   $2: Show output flag, "-t" to show output, empty otherwise
function run_problem() {
    local problem_number=$1
    local show_output=$2
    local problem_file="src/problems/problem${problem_number}.rs"

    # Check if the problem file exists
    if [ ! -f "$problem_file" ]; then
        echo "Problem file not found: $problem_file"
        exit 1
    fi

    # Generate the mod content
    local mod_content="pub mod problem$problem_number;"

    echo -e "$mod_content" > src/problems/mod.rs

    # Generate the main.rs content using the generate_content function
    local main_content
    main_content=$(generate_content "$problem_number")

    # Get the current time
    local time_executed
    time_executed=$(date +"%a, %Y-%m-%d %H:%M:%S %Z")

    # Check if "Last Executed:" already exists in the problem file below the "// Link:" line
    if grep -q "// Link:" "$problem_file" && grep -q "// Last Executed:" "$problem_file"; then
        # If "// Link:" and "// Last Executed:" both exist, replace the "Last Executed" line with the new timestamp
        sed -i "s|// Last Executed:.*|// Last Executed: $time_executed|" "$problem_file"
    else
        # If "// Link:" does not exist or "// Last Executed:" does not exist, append the "Last Executed" line below the "// Link:" line
        sed -i "/\/\/ Link:/a // Last Executed: $time_executed" "$problem_file"
    fi

    # Replace the main.rs content with the generated one
    echo -e "$main_content" > src/main.rs

    # Compile and run the specific problem file using cargo, redirect stderr to /dev/null to disable build output
    local output
    if [ "$show_output" = "-t" ]; then
        output=$(cargo run "$problem_number")
    else
        output=$(cargo run "$problem_number" 2>/dev/null)
    fi

    # Echo the output with the "Output" label
    local bold
    bold=$(tput bold)

    local normal
    normal=$(tput sgr0)

    local blue_color="\033[38;2;137;180;250m"
    local input_color="\033[38;2;180;190;254m"
    local output_color="\033[38;2;166;227;161m"
    local title_color="\033[38;2;242;205;205m"
    local reset="\033[0m"

    local title
    title=$(extract_title "$problem_file")

    local category
    category=$(grep -Po "(?<=\/\/ Category: ).*" "$problem_file")

    local top_title="Problem $problem_number | $title ($category)"
    local top_title_edited="${bold}${blue_color}Problem ${problem_number}${reset} | ${normal}${title_color}${title}${reset} (${category})"
    # local top_title_edited="${bold}${color}Problem $problem_number │ ${normal}$title"
    local top_title_length=${#top_title}

    # Calculate the number of dashes needed for the separator line
    local separator_length=$((top_title_length))  # 5 accounts for "Problem ", " | ", and " │ "

    # Extract the input from the problem file (if available)
    local input
    input=$(grep -Po "(?<=\/\/ Input: ).*" "$problem_file")

    local category
    category=$(grep -Po "(?<=\/\/ Category: ).*" "$problem_file")

    echo -e "$top_title_edited"
    printf "─%.0s" $(seq 1 "$separator_length")
    echo
    if [ -n "$input" ]; then
        echo -e "[${input_color}${bold} Input ${reset}]${normal} $input"
    fi
    echo -e "[${output_color}${bold} Output ${normal}${reset}] $output"
}

# Function to run all problems
function run_all_problems() {
    local test=$1
    # Loop through all problem files and run the corresponding problem
    first_file=true
    for problem_file in src/problems/problem*.rs; do
        # Extract the problem number from the file name (e.g., "src/problems/problem123.rs" -> "123")
        problem_number=$(basename "$problem_file" | grep -oP "(?<=problem)\d+(?=.rs)")
        # Check if it's the first iteration
        if [ "$first_file" = true ]; then
            first_file=false
        else
            echo  # Print a newline before running the next problem
        fi

        # Call the run_problem function for each problem file
        run_problem "$problem_number" "$test"
    done

}


# Check if the correct number of arguments is provided
if [ "$#" -lt 1 ] || [ "$#" -gt 2 ]; then
    echo "Usage: $0 <problem_number> [-t]"
    exit 1
fi

# Get the problem number from the first argument
problem_number=$1

# Check if the "-t" flag is provided as the second argument
if [ "$#" -eq 2 ] && [ "$2" = "-t" ] && [ ! "$1" = "all" ]; then
    # Call the run_problem function with the provided problem number and the "-t" flag
    run_problem "$problem_number" "-t"
elif [ "$1" = "all" ]; then
    if [ "$2" = "-t" ]; then
        run_all_problems "-t"
    else
        run_all_problems
    fi
else
    # Call the run_problem function with the provided problem number (without the "-t" flag)
    run_problem "$problem_number"
fi
