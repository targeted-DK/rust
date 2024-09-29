First Section
1. CSE 542 Fall 2024 Lab 1

2. Donggyu Kim(DK), donggyukim@wustl.edu

3. 
The program was designed to read several files based on given configuration file initially.
'cargo run' command line takes in required parameters, which is crucial to start this program.
For example, cargo run hamlet_ii_2_config.txt, which includes the configuration file name that includes the names of files we will read later,
is a starting point for this program. "whinge" is an extra parameter that prints out some warning lines when the program encounters badly formed lines in the configuration file.

After parsing necessary lines from command args, we call script_gen() function in the main().
This is the start of the program as it parses text files we need to read.
read_config() function called within script_gen() function parses text files we need to read.
We then call process_config(), which is the heart of this program - this reads every lines from each txt file using grab_trimmed_file_lines and 
converts necessary data(line number, character name, line) into tuple object and into vector so we can sort then based on line number later.

The important aspect of this program is the mutability of a passed parameter in the function.
This is crucial to the final output of this program because existence of single mutable reference does not allow 

To sum up, main() calls, script_gen() function, which calls two functions - read_config() and process_config(), in which the first function processes txt files we need to read,
and the second function reads each lines from each txt and converts each lines into appropriately formatted tuples in a vector.


4. Insights
I had issue with the ownership issues of an object, especially between String and str. Some methods methods of libraries returned &str, so the use of .to_string() to crucial
to parse data into correct format. 
Also, in order to avoid ownership issues, deepcopying some objects using .clone() was easy way to solve ownership issues.
I also loved how the subtle difference of using Result<> as a return value impacts design of the program.
The use of Result<> can output the chain of successes and errors, which if used correctly, allow the programmer to find where the errors are. 

The use of Ordering::SeqCst which works as a on/off switch to trigger certain functinality was impressive. It could potentially prevent data races when 
multiple files are read at the same time using multiple threads.

The best part, which was not intuitive first, but later I found effective was the use of &mut. It really made debugging process easy because I knew which paramter can
or cannot be modified so it was time-saving to find which variable is being modifed where.

Second Section

Instructions how to build and run this program
1. unzip lab1.zip file using 'unzip lab1.zip'

2. cd into lab1 folder.

3. run 'cargo build'

4. move necessary files into debug folder using 'mv ./files/* ./target/debug'

5. cd into target/debug folder

6. run ' ./lab1 hamlet_ii_2_config.txt' or ' ./lab1 hamlet_ii_2_config.txt whinge'

Third Section
I tested this program by adding non existing file names in the config file with / without non existing character name.
For example:
Hamlet Prince of Denmark ACT II Scene II A room in the Castle by William Shakespeare
Guildenstern Guildenstern.txt
King King.txt
Queen Queen.txt
Rosencrantz Rosencrantz.txt
Rosencrantz crantzRosen.txt
test test.txt
test

I tested this program by adding non-usize line with / without character lines
For example, in the Rosencrantz.txt,
a 5
27 Both your majesties 
28 Might, by the sovereign power you have of us, 
29 Put your dread pleasures more into command 
30 Than to entreaty.
k k
k 

I tested program by providing few or too many command line arguments.

