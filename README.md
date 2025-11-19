# Binary Search Tree

This is a Binary Search Tree I made in rust to expand my understanding of rust and also because this is a really cool thing to build!

## Explain the project

The tree is based on a to-do list but that is not important. 
We have a file called 'tasks.txt' that looks kind of like this:
```text
6|program|DONE
2|eat|TODO
4|sleep|TODO

// This is the stucture:
priority|description|completed
```

So when we add a file in the program there is a new line added in this file. When we check a task the text changes from 'TODO' to 'DONE' and also vice versa.

Oke now we'll use the project!

```bash
✓ Tasks loaded from tasks.txt
[✓] Priority 6: program
[ ] Priority 4: sleep
[ ] Priority 2: eat
> 
```
This looks like what you see when start the program. Notice that the order the tasks are loaded in are in descending order. So in the program we order on priority.

### Commands

To use the program you need to know some commands.

 **`ls`**
List all tasks **ordered by priority**.

**`find <priority>`**
Find and display the task with the given priority.

**`add <task>`**
Add a new task with a specified priority.

 **`rm <priority>`**
Remove the task with the given priority.

**`check` / `uncheck`**
Mark a task as **completed** or **not completed**.

**`tree`**
Display the task tree in a **horizontal** format.

**`vtree`**
Display the task tree in a **vertical** format.

So know you know the commands to things inside the program.

## Live Tree Mode

I made a different mode in the program to see you tree live in the terminal! So when you make changes you can see them directly. I think this is a little bit cool.

To go into this mode you type:
```bash
> live
```

Then the screen is cleared and you will see a tree displayed with underneath a prompt that looks like this:

```bash
		┌── [5]
	┌── [5]
── [6]. ┌── [4]
	└── [3]
		└── [2]
live > 
```

So you here do the same operations like before but now they are displayed directly!
