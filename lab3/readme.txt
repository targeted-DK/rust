Section One.

1. CSE 542 Fall 2024 Lab 3

2. Donggyu Kim(DK) donggyukim@wustl.edu

3. Overview of lab3
This lab is built on top of lab2, by implementing multithreads to split the work into multiple threads. The server struct was created to add
extra functionality(server client and distributed IO) in this lab - that when the necessary files(such as configuration files) does not exist in the local level, the program
checks whether the same file exists in the server. 


4.
Coding a server and test client components was initially tricky to me due to many of methods/objects used had to be gracefully unwrapped to make sure it workss.
Many components of Rust language we've learned throughout the semester now seems to make sense, espeically the program's strict rules on a struct's ownership.

Section Two.

1. unzip the zipped folder.
2. I have already added necessary files to both lab3client cargo, and lab3server cargo just in case. All you have to do is,
- use 'cargo build' on all three rust cargos(lab3client, lab3server, lab3testclient) to make sure that code compiles again.

3. Running instructions.
- Before you use lab3client or lab3testclient, make sure you run lab3server so that the server is LIVE before you want to script a play.
    - You can run lab3server by running './lab3server 127.0.0.1:7777' on ~lab3/lab3server/target/debug folder. 
    - You may change ip address or port number depending on your network configurations.
- When the server goes on, you will see message that says similar to : "waiting for connections at address : 127.0.0.1:7777 Server running...."
- You can test whether the server listens to any incoming connections by
    - Move into directory ~/lab3/lab3testclient/target/debug folder.
    - Run command "./lab3testclient 127.0.0.1:7777 partial_hamlet_act_ii_script.txt"
    - If the compiled lab3testclient successfully runs and the running lab3server successfully listens, you will see the message that is similar to
        "Connecting to: 127.0.0.1:7777 Token: partial_hamlet_act_ii_script.txt" on your testclient
    - The server side will print, "new client: addr : 127.0.0.1:38718 Connection received from 127.0.0.1:38718 Server running.... The string: partial_hamlet_act_ii_script.txt"

- Now we know that the server is successfully running to work as a backup server to script a play using lab3client,
- You can run script a play by like we did in lab2.
    - Move into directory ~/lab3/lab3client/target/debug folder.
    - Run command "./lab3client partial_hamlet_act_ii_script.txt" or ./lab3client partial_macbeth_act_i_script.txt"


Section Three.
1. My inital problem was that I was trying to test whether the server works on the shell linux machine on campus. To test the server and the testclient,
I opened up two different instances on linux machines and qlogined in each of them. I figured it out later that if you run two different shells logged in using qlogin,
they are technically on the different machine so that the test client could not locate the running server because they were running on the different machine. 

2. I had trouble with type issues related to the last part of instruction 13. When the input string does not contain properly forammted TCP connection string,
we had to use that as an input file name or if it is a proper TCP connection format string, we had to use that to connect to the running server by parsing 
ip address and the port number. The problem was that we had to return a newly initialized BufReader, intialized either with a File or TcpStream. I tried to use generics
<R> type since File / TcpStream struct was not decided until the runtime. This was a bit of pain to figure out because if I tried to modify get_buffered_reader's type to some R,
I had to modify grab_trimmed_file_lines as well, since grab_trimmed_file_lines uses get_buffered_reader, which ended up complicating the code too much.
After a bit of searching, I found that using Box::new() allows lazy initialization of BufReader with a type. 

3. My main issue was making the server work but mostly it had to do will trivial issues caused by lack of orderness in my code.
I would often forgot that I deleted some portion of code from lab2 (such as self.players.push(player)) or syntax issues that dealt successful return / return data types of a methods
that would make code produce wrong output, which ended up taking a long time to debug. 

4. Another issue was due to misunderstanding of Arc<Mutex> variables. Misinterpratation of how Arc mutex variables actually refers to same object mixed with lifetime issues combined with
child threads, the code became a chaos due to so errors from everywhere. 

My tests
1. Testing was mostly about making the server work. I tested with different ip addresses and port names to make sure that the connection was stable.
2. Note to professor that I had a hard time with testing server because I was testing with two different qlogined terminals on school machine....(I thought something was wrong with my code)
3. I added different configuration file names by adding ip address and port name, for example, 
net:127.0.0.1:1234:hamlet_ii_1b_config.txt instead of hamlet_ii_1b_config.txt. The local and remote file names was retested with wrong formats and wrong names to check
whether the program filters wrong inputs.
Different port/ip addresses were tested to see if the connection is stable only with proper addresses.

Thank you very much for everything throughout the semester! - DK
