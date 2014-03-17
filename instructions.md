Getting Started
======

Before continuing with this assignment, **one member of your team** should:

1. Set up the private repository named 'cs4414-ps4'.

2. Add your teammate(s) and 'cs4414uva' as the collaborators.

3. Clone the empty private repository to your working environment. Instead of mygithubname below, use your github username.

	    git clone https://github.com/mygithubname/cs4414-ps4.git

4. Get the starting code for ps4.

	    git remote add course https://github.com/cs4414/ps4.git
	    git pull course master
	    git push --tags origin master

5. Get the starting IronKernel code:

	    git clone https://github.com/wbthomason/ironkernel.git
	    cd ironkernel
	    git submodule update --init

6. After finishing these steps, you should now be able to run IronKernel by executing:

	    make run

You should see the IronKernel logo, and a QEMU window. The window as of now is only an echo shell - it prints whatever you type into it! In fact, it even has some extra functionality, like handling the backspace and enter key correctly. (Many of the non-printing function keys will freeze your terminal!) 

We want to make IronKernel behave more like a terminal so we can eventually run zhtta on it! First we'll have to have a prompt.

> Problem 1. Modify `kernel/sgash.rs` to make it propmt you with sgash whenever the user types enter. (The actual change is simple, but the point of this question is to get you starting to explore the IronKernel code.) 

Strings
------

The first step to having a kernel working is for it to remember what you typed into it. However, libraries like string formatting are not provided to you on a standalone piece of hardware. In fact, the only thing you get is raw data types like u8, uint, and char. So we have to first implement a simple string library to more easily manipulate what the user types.

> Problem 2. Modfify `kernel/sgash.rs` to echo whatever you type into the prompt back to you on the second line.

Commands
------

After we have a string library going, we'll want to allow users to input and output commands. 

> Problem 3. Code in some basic commands, so that the system recognizes echo, ls, cat, cd, rm, mkdir, pwd, and a new ficticious command wr for writing a string to a file. We will implement these commands in a later step.

IO
------

After some thought you have discovered the terminal's color scheme is lacking. Let's change the color scheme!

> Problem 4a. Change the color scheme of IronKernel to something cooler. Bonus points if you allow users to change their own colors. Be prepared to explain how characters are drawn to the screen during the demo. (Hint: look in `arch/arm/io`)

### Fonts

It's not easy to write fonts for IronKernel. Charles Eckmann has decided to provide a bitmap font pack in `arm/arch/io/font.rs`. However, we want to add a bit of a flair to IronKernel, so let's change a character or two to our liking.

> Problem 4b. Change the @ symbol to have an 'R' inside it instead of an 'a'.

Interrupts
------

.

Filesystem
------

Time to put it all together. The simplest filesystem just implements a tree, where each branch node is a directory and each leaf node is a file. For now, we'll only support text files. In addition, we'll keep all our files on RAM. Who needs to shut down anyway.

> Problem 6. Write a filesystem.

However, a filesystem is useless without the ability to travese, write, and read from it.

> Problem 7. Implement ls, cat, cd, rm, mkdir, pwd, and wr for your filesystem. These commands do not have to emulate their more complex bash counterparts. wr will take a file as its first argument and a string as its second argument. It will write the string to the file.

Improving IronKernel
------

For the last problem, your goal will be to improve IronKernel in some way.

> Problem 8. Modify your kernel to make it more usable as a kernel. 

Some suggestions include getting the arrow keys to work, getting pipes to work, and implement ext4's filesystem paradigm, which makes less fragmented files but gives up some speed in accessing files sequentially.
