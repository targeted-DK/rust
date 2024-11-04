SECTION ONE 

1. CSE 542 Fall 2024 Lab 2
2. Donggyu Kim (donggyukim@wustl.edu)

3. This lab is designed to implement the idea of modularity into our program by splitting our original lab1 code into necessary components. In this lab, there are 3 main structs that consists this program,
which are Play, Player, and SceneFragment. It is straightforward by name, that a single Play is consisted of multiple SceneFragments, and each SceneFragment is consisted of Player Structs, and
each Player Struct includes important information like names and lines for each player.
How this program runs is that based on title/configuration file names written in a starting file, 'partial_hamlet_act_ii_script.txt', the program detects which configuration files to read and open for each scene,
and each configuration file has information about which files to read/open, for each player in a scene.
So there is a one-to-one correspondence of config files with scene structs and player structs. 
Therefore, each struct has multiple functions that assists the process of converting information given from higher granularity module(e.g. Play->SceneFragment->Player), such as process(), read_config(), add_config().

In this lab, I spent a lot of time debugging trying to figure out which function or line was causing a abnormal output. Sadly, I did not encounter something insightful aspect of rust programming for this lab, other than
error messages, which are really nicely designed in Rust....

Modules
Keeping the modules in clean manner was straightforward - I had to check which const/variables were used in single file or multiple files. Other than that, any places we used to print error messages
were converted to eprinln! syntax, which were mostly within if block for whinge mode. 

Structs
So to refactor lab1 code, I created Play struct in play.rs file and Player struct in player.rs file. Play struct is an object that includes play title and players of the given scene, and
Player struct has information of player name, lines and texts by each player, and an index to keep track of which lines that player owns that have been printed to the output.  
I had a little trouble interpreting the instructions because having no parameter for recite function as a 'method' means implict use of self as a parameter, which took me awhile to figure out because
I interpreted as static method for play Struct rather than an instance. Other than that refactoring was straightforward when I realized how script_gen functions will be divided into two parts. 
In general, my misinterpretation of associated functions / associated methods / with or without &self as a paramter caused huge pain while debugging. I ended up printing output line by line to debug because
the intial design of the functions were wrong.

Return Wrapper
I think creating Return Wrapper to replace Result<> was straightforward, but I had issues because of my syntax mistake - I tried to add new() within Termination trait implementation.
Also there were some issues while debugging - although main() returns Return Wrapper, functions used in main should return Result<>, to use match syntax to split success and error cases, which then could be
wrapped with Return Wrapper to be return in main().


Scene Fragments
"Ordering" struct was used both is atomic and cmp module so I had to make sure explicitly that which Ordering should be used.
Scene Fragments were really really a pain - although the modularity of SceneFragment is clean and efficient(and makes sense in terms of how scenes are written as a script in real life),
debugging was a little complicated on my side due to wrongly interpreting the origin of wrong outputs, as there were multiple functions nested in between modules.
We had similar names for associated functions used in SceneFragment and Play structs - and I kept getting forgetting which one is which while trying to find the origin of wrong output.
To me, the intialization of SceneFragment struct with a given title bit of trouble because a lab 1 code that was not erased unknowingly kept replacing scene title with a config file title. 
Using several booleans to maintain which players are done speaking and which line was done spoken in recite() seemed intially ok, but I tried to use the double loop for recite() to print line by line from each players.
I ended up using a vector to maintain which players are done speaking within the double loop because sometimes players were marked done while they were not (couldn't figure out why though, using double nexted loop with few boolean conditions seemed fine to me).
I had an error where the only first scene was being printed not the second one. I initially thought it had to do with Fragments vector in Play or Enter / Exit functions but recite() function was not properly ended due to boolean values.
Also pushing a fragment into fragments vector was a move operation so after pushing the fragment, I had to access fragments vector's last element to call fragment.prepare()


Testing
I spent more time on debugging and rather than testing - I guess for this lab debugging was more of a testing procedure to me. Often whenever I ran the cargo run partial_hamlet_act_ii_script.txt command,
there were cases like
1) Only one scene prints
2) Only one lines from each player prints
3) Title of the scene is replaced by configuration file names
4) not all players' lines are printed and moves to next scene

Few testings I did after got code right was to see if the program properly returns errors if there were wrong config file names or if it skips lines of player that are not in correct format.


SECTION TWO  
