Title: Problem Set 4 - IronKernel
Date: 2014-03-18
Category: PS
Tags: Problem Sets, Kernel
Status: draft
Slug: ps4

   <div class="due">
   Thursday, 3 April at 11:59pm
   </div>

## Purpose

The goal of this problem set is for you to develop a deep understanding
of how operating systems work by hacking on a small kernel.  

All of the programs you have written until now in this class (and
probably in your life, unless you have had unusual experience outside
your classes) have run at user-level, with limited access to machine
resources.  They have also depended on lots of other programs to run,
including an operating system.  

A kernel does not rely on any other programs to run.  It is the program
below all other programs, and it has complete access to all machine
resources.  Programming a kernel gives you unlimited power (at least,
not limited by any underlying operating system, but only by the limits
of your physical processor), but also poses many challenges since you
don't have any other programs to build on!

For this problem set, we will provide a very limited starting kernel for
ARM processors implemented in Rust:
[IronKernel](https://github.com/wbthomason/ironkernel).  IronKernel was
developed starting from [rustboot](https://github.com/pczarn/rustboot),
as a [project in last semesters cs4414
class](http://rust-class.org/0/pages/final-projects.html) by Kevin
Broderick, Alex Lamana, Zeming Lin, John Stevans, and Wil Thomason
(Alex, Wil, and Zeming are current cs4414 TAs!).

IronKernel doesn't provide many things you would expect in an operating
systems today so is a long way from being able to host your Zhtta
server, but it is enough for you to get some fun experience hacking a
kernel and we hope (with your help!) it will one day become an operating
system one could use to host a web server.  One of the things IronKernel
currently doesn't provide is any kind of file system, which is one of
the things you will implement for this assignment.  The file system you
will implement will be an in-memory file system, so there is no need to
write a disk driver (but the files are not persistent &mdash; everything
is lost when you shut down).

## Setting Up your Environment

Since developing on bare metal is difficult, most kernel development is
done in an emulator.  We will use the
[QEMU](http://wiki.qemu.org/Main_Page) emulator to emulate an ARM
processor.  In addition to installing the emulator, you will also need
to install an ARM cross-compiler.  You'll be running the compiler on
your x86 processor, but it will be producing binaries for the ARM
processor which you will run on the QEMU emulator.

You can set up your enviornment for this by either:

1. Executing the shell commands [here](|filename|./setup.md).

2. Downloading the provided VirtualBox VM image:
[http://www.rust-class.org/static/cs4414-ubuntu-12.04.2-32bit-vdi-ps4-env.zip](http://www.rust-class.org/static/cs4414-ubuntu-12.04.2-32bit-vdi-ps4-env.zip).
This includes all the tools you need for ps4, but may take a few hours
to download.  (Note that standard unzip may not be able to unzip this
file.  On Mac OS X, [Keka](http://www.kekaosx.com/en/) works.)

If you are ambitious, you can try to set things up on another platform,
but be forewarned this could be a time-consuming and frustrating effort
since many of the required packages depend on particular versions of
other packages that are installed.


### Collaboration Policy

For this problem set, you are **required to work in a team of three or
  four people** (except in cases where you were notified based on your
  PS3 teamwork that you should work alone for PS4, or where you make
  your own successful argument before **21 March** that it is better for
  you to work alone or in a team of two).

Your teams should work together in a way that is efficient and
collaborative, and **ensures that all team members understand everything
in the code you submit**.  As part of the grading for this assignment,
you will do a short demo with one of the course staff, at which all team
members will be expected to be able to answer questions about how your
code works.

Please note that only one team member should create the private
repository for this problem set.  The other members should work in the
same repository as a collaborator.

In addition to working directly with your teammates, you should feel
free to discuss the problems, provide coding help, and ask for help with
any students in the class (or anyone else in the world, for that
matter), so long as you don't to it in a way that is detrimental to your
own or anyone else's learning.  You can do this in person, using the
course forum (including comments at the end of this page), using the
`#cs4414` and `#rust` IRC channels, or any other communication medium
you find most effective.

# Getting Started

Before continuing with this assignment, **one member of your team** should:

1. Set up the private repository named 'cs4414-ps4'.

2. Add your teammate(s) and 'cs4414uva' as the collaborators.

3. Clone the empty private repository to your working environment. Instead of mygithubname below, use your github username.
```bash
   git clone https://github.com/mygithubname/cs4414-ps4.git
```
4. Get the starting code for ps4.
```bash
    git remote add course https://github.com/cs4414/ps4.git
    git pull course master
    git push --tags origin master
```

5. In the ps4 directory, get the rust-core module by doing:
```bash
    git submodule update --init
```

6. After finishing these steps, you should now be able to run IronKernel by executing:
```bash
    make run
```
You should see the Ironkernel logo, and a QEMU window.

The window as of now is only an echo shell - it prints whatever you type
into it!  This may not seem very exciting to you if you are used to huge
and luxurious operating systems like Linux, Mac OS X, and Windows that
allow users to do things like run programs, connect to the Internet, and
use a disk drive.  But, if you think about what it takes to go from the
bare hardware to something that can print when you type, you should find
it very exciting!  In fact, it even has some extra functionality, like
handling the backspace and enter key correctly. (Many of the
non-printing function keys will freeze your terminal, so be glad you are
running in a simulator!)

We want to make Ironkernel behave more like a terminal so we can
eventually run zhtta on it!  (We don't expect anyone to get that far
with it this semester though.  That said, feel free to surprise us!)  

As noted in Exam 1, running a shell in kernel-space if probably a really
bad idea.  (Its a less bad idea if it is mostly programmed in safe Rust
code than in unsafe code, since at least you get the benefits of Rust's
software-based memory protection for the shell implementation.)  But,
since IronKernel does not yet have the ability to create a new process
and run code at user-level, for this assignment you will be implementing
a shell that runs are part of the kernel.

# Hacking the Kernel

First, make some seemingly trivial changes to the kernel to get started
with kernel hacking.  Each of these changes only involves one or two
lines of code, but the goal of these questions is to get you comfortable
with kernel-level programming and understanding some aspects of the
IronKernel code.

   <div class="problem">
**Problem 1.** Modify `kernel/sgash.rs` to make it prompt you with `sgash> `
  whenever the user types enter. (The actual change is simple, but the
  point of this question is to get you starting to explore the
  Ironkernel code.)
  </div>

   <div class="problem"> 
**Problem 2.** Change the color scheme of Ironkernel to something
  cooler. Bonus points if you allow users to change their own colors. Be
  prepared to explain how characters are drawn to the screen during the
  demo. (Hint: look in `arch/arm/io`)
  </div>

   <div class="problem"> 

**Problem 3.** After a particularly long and painful night of
  kernel-hacking, you are starting to feel like everything is
  upside-down.  To correct this, modify the code so characters are
  printed upside-down (that is, the letters should appear normal if you
  turn the screen upside-down).  (Hint: this only involves changing one
  line of code.)

</div>

(After you complete problem 3, you may comment out your code and go back
to rightside-up characters, but that's up to you which you prefer.)

# Strings

The first step to having a working shell in our kernel is for it to
remember what you typed into it. However, libraries like string
formatting are not provided to you on a standalone piece of hardware. In
fact, the only thing you get is raw data types like u8, uint, and
char. So we have to first implement a simple string library to more
easily manipulate what the user types.

   <div class="problem"> 
**Problem 4.** Modfify `kernel/sgash.rs` to echo whatever you type into the prompt back to you on the second line.
   </div>

# Commands

After we have a string library going, we'll want to allow users to input
and output commands.

  <div class="problem"> 
**Problem 5.** Code in some basic commands, so that the system recognizes
  `echo`, `ls`, `cat`, `cd`, `rm`, `mkdir`, `pwd`, and a new ficticious command `wr` for
  writing a string to a file. We will implement these commands in a
  later step.
  </div>

After problem 5, your shell should display an error message for any
unrecognized command, provide the original echo functionality for
`echo`, and do nothing for the other commands.

<!--
### Fonts

It's not easy to write fonts for Ironkernel. Charles Eckmann has decided
to provide a bitmap font pack in `arm/arch/io/font.rs`. However, we want
to add a bit of a flair to Ironkernel, so let's change a character or
two to our liking.

> Problem 4b. Change the @ symbol to have an 'R' inside it instead of an 'a'.
-->

Filesystem
------

In order for our commands to actually do anything we need to add a filesystem.

The simplest filesystem just implements a tree, where each branch node
is a directory and each leaf node is a file. For now, we'll only support
text files. In addition, we'll keep all our files in memory.  Writing a
driver for a disk is hard and depends on the physical device, but we'll
assume the owner of a machine running IronKernel is rich enough to
purchase all the memory they need and won't every shut down their
machine, so an in-memory filesystem is sufficient!

A simple filesystem provides these calls:

 -- `read_file(file)` - returns the string stored in a given file
 -- `write_file(file, string)` - writes the given string to the given file
 -- `create_file(directory, name)` - creates a file with the given name in the given directory
 -- `delete_file(directory, name)` - deletes the file with the given name in the given directory
 -- `get_file(directory, name)` - gets the file with the given name belonging to the specified directory
 -- `list_directory(directory)` - returns the list of files and directories contained in the specified directory
 -- `create_directory(parent, name)` - creates a directory with the specified name under the given parent directory
 -- `delete_directory(directory)` - deletes the given directory if and only if it is empty
 -- `get_directory(parent, name)` - gets the directory with the given name belonging to the specified parent

   <div class="problem"> 
**Problem 6.** Implement a filesystem that provides at least the calls described above.
   </div>

Once the filesystem calls are implemented, you can use them to implement commands that users could use to manipulate files in your shell.

   <div class="problem"> **Problem 7.** Implement `ls`, `cat`, `cd`,
`rm`, `mkdir`, `pwd`, `mv`, and `wr` as built-in shell commands that use
your filesystem. 
   </div>

These commands should provide the minimal basic functionality of their
unix counterparts, but do not need to handle any options or more than
pathnames as their inputs (one pathname for `ls`, `cat`, `cd`, `rm`,
`mkdir`; none for `pwd`; and two for `mv`).  The `wr` command should
take a pathname as its first input and a string as its second
argument. It will write the string to the file.  Each of these commands
should be implemented as a function that uses your filesystem API calls.


# Memory Management

Memory in Ironkernel is done using the Buddy Blocks allocation
system. This system treats the whole memory as something akin to a
binary tree, where each requestion for a segment of memory would involve
traversing down the tree until you find a block just big enough to
contain it. For example, if your block of memory was 128kb large and you
request for an allocation of 26kb, we would divide the memory space into
something akin to:
   <pre>            
          32kb    32kb        64kb
        |xxxxxxx|------|----------------|
   </pre>

and the user would be guarenteed the first block for their allocation.

We have provided intial code for the Buddy Block allocation system
(which was enough for you to implement the file system using it), but
there's a fatal bug in it! Whenever space is reclaimed, our code fails
to reclaim it correctly.

   <div class="problem">
**Problem 7.** Identify the bug with the Buddy Block allocation system
  and fix it. (Hint: What is the correct behavior of the memory manager
  when two adjacent blocks are both reclaimed and free?)
   </div>

# Improving Ironkernel

For the last problem, your goal will be to improve Ironkernel in some
way.  
   <div class="problem"> 
**Problem 8.** Improve your kernel in some interesting way.
   </d iv>

Some suggestions include getting the arrow keys to work, getting pipes
to work, and implementing ext4's filesystem paradigm, which makes less
fragmented files but gives up some speed in accessing files
sequentially.

If you want to do an ambitious improvement, you can get started on it
for this problem, but make finishing the improvement your final project.

### Submission, Benchmarking Competition, and Demos

There are three parts to submitting PS4:

1. Submit the **PS4 Submission Form** (by 11:59pm on **Thursday, 3 April**).

2. Sign-up for a **PS4 Demo**.  You should sign-up for a
demo time by **4:59pm** on **Tuesday, 1 April**.

3. Within 24 hours of finishing your demo, each team member shoud
invidually submit the **PS4 Assessment Form**.  

<div id="disqus_thread"></div>

<script type="text/javascript">
        /* * * CONFIGURATION VARIABLES: EDIT BEFORE PASTING INTO YOUR WEBPAGE * * */
        var disqus_shortname = 'rust-class'; // required: replace example with your forum shortname
    var disqus_url = 'http://www.rust-class.org/pages/ps4.html';

        /* * * DON'T EDIT BELOW THIS LINE * * */
        (function() {
            var dsq = document.createElement('script'); dsq.type = 'text/javascript'; dsq.async = true;
            dsq.src = '//' + disqus_shortname + '.disqus.com/embed.js';
            (document.getElementsByTagName('head')[0] || document.getElementsByTagName('body')[0]).appendChild(dsq);
        })();
</script>
<noscript>Please enable JavaScript to view the <a href="http://disqus.com/?ref_noscript">comments powered by Disqus.</a></noscript>
<a href="http://disqus.com" class="dsq-brlink">comments powered by <span class="logo-disqus">Disqus</span></a>


   <div class="credits"> IronKernel was developed starting from
[rustboot](https://github.com/pczarn/rustboot), as a [project in last
semesters cs4414
class](http://rust-class.org/0/pages/final-projects.html) by Kevin
Broderick, Alex Lamana, Zeming Lin, John Stevans, and Wil Thomason.
Alex, Wil, and Zeming designed and wrote most of this assignment, with
some help from David Evans.
   </div>

