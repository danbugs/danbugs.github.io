---
layout: post
title:  "Git Tutorial"
category: git
---

## Local

First, let's download git [here](https://git-scm.com/downloads).

You could also obtain git through:

1. Desktop applications such as GitHub Desktop or the Bitbucket equivalent, and
2. Xcode.

Before we create our first local git **repository**, it is good practice to establish your username and user e-mail in the system. You can do so by running `git config --global user.name <your name>`[^1] and `git config --global user.email <your email>`.

After that, we are good to create our first local git repository:

1. Open the terminal (`Cmd + Space` and type `Terminal`),
2. Navigate to a convenient location in your computer (`cd <path>`), and
3. Make a directory (folder) where we will initiate the git repository (`git init`).

Just for test purposes, let's start an `index.html` file and add some basic content to it such as:

```
<!DOCTYPE html>
<html>
	<head>
	</head>

	<body>
		<h1>H1</h1>
		<p>P1</p>
	</body>
</html>
```

Now, you what it means when we say git is a version control system. Let's **commit** our work.

1. Run `git status`. This will show you that the git recognizes there have been changes to the repository.
2. Run `git add .`
3. Running `git status` again. Now, you should see that the changes are green instead of red. This means they have been **staged**.
4. All the staged content is ready to be committed. However, before we do so, run `git log --oneline`. This command shows us all the commits that have been done to this repository.
5. Run `git commit -m <message>`.
6. Run `git log --oneline` again. You should see that there is one commit.

Now, let's make a couple of changes to our index.html file. Add `<p>P2</p>` below `<p>P1</p>`.

Once again, let's commit our changes:

```
1 git status
2 git log --oneline
3 git add .
4 git commit -m <message>
```

Suppose, I did not like this latest update and I wanted to go back to how it was before. All you have to do is run `git checkout <commit id>`[^2]. After this, you should see changes to your `index.html` file.

To go back, just repeat the command `git checkout <commit id>` with the `<commit id>` from the last commit.

Let's make a couple more change to our project. 

1. Create a css folder inside of our main directory.
2. Create a `styles.css` file inside of our css folder.

To the styles.css folder, add the following:

```
body
{
	font-family: monospace;
}
```

Let's add another paragraph to our `index.html` (i.e. `<p>P4</p>`) and save it.

I would like to show you that the `git add` command can also be used for specific files. For example, suppose I did not want to commit my change to the `index.html` file:

1. Run `git status` so we can see our current state.
2. Run `git add css/`. This will make git ignore the changes to the `index.html` file, staging only the content from inside the `css/` folder.
3. Run `git status` to check that only the content from `css/` has been changed [^3].

Now, suppose that was a mistake, I actually never wanted to stage that. To unstage it we can run `git reset HEAD`. Run `git status` to verify [^4].

Similarly to how we used `git add` on specific files, we could also use `git checkout` on specific files. Just do `git checkout <commit id> <specific file>` to check a file from a specific commit. 

The last thing I would like to talk about is a very special file we can use called `.gitignore`. This file allows us to tell git specific files we would **never** like to commit.

Let me show you how to use it. To start, I want to add a testing file to our project. Let's call it `virus.css` and place it in the main project directory.

Inside of `virus.css`, just write:

```
body
{
	background-color: #000;
}
```
Run `git status`, you should see that git is recognizing `virus.css` as an unstaged file.

After that, go to your project folder and create a file named `.gitignore`[^5]. Depending on the system you are running, it might give you a warning about starting file names with a `.` but you can ignore that.

Using your editor, open the `.gitignore` file and write `virus.css` in the first line.

Run `git status`, you should see that git is **only** recognizing `.gitignore` as an unstaged file.

Run `git add .` and `git commit -m <message>` to commit your changes. Run `git ls-tree --name-only master` to see that `virus.css` is not present in our commit.

## Online

In this step, we will be moving all our local git repositories online so they can be accessed by multiple people. For this, you will need either a GitHub or BitBucket account. 

If you don't already have one, I would recommend joining GitHub. You can do it [here](https://github.com/join).

After creating your account and verifying your e-mail you will want to do the following: 

1. Click on **Start Project**,
2. Name your repository,
3. Select **Public** if you have the free plan,
4. No need to add a `README`, `.gitignore` or license.

After creating the repository, you should be taken to a **Quick setup** page where you can see an HTTPS URL. Copy this link.

Now, heading back to terminal [^6], run `git remote add origin <link you copied>`. This will connect your local repository to your online repository.

The last thing we have to do is just **push**[^7] all the files. Run `git push -u origin master`.  

Now, if you reload your GitHub/Bitbucket page. You should be able to see all your files there.

## Troubleshooting

An issue I see happening quite often is that the `HEAD` detaches from the master branch, which renders you unable to commit new files because everything seems up-to-date. To fix this, do the following:

1. Run `git log --oneline`. This will show you where `HEAD` is. You will need that commit id.
1. Run `git branch temp` to make a new branch called temp and `git checkout temp` to move to that branch [^8].
2. Run `git ls-tree --name-only temp`. This will allow you to see, which files are currently in the temp branch. If all the correct files are listed, you are good. If not, just run `git add .` and `git commit -m <message>`.
3. Run `git checkout master`. This will take you again to the master branch.
4. Run `git merge temp`. This will merge your up-to-date `temp` branch with `master`.

Problem fixed.

[^1]: Any place you see `<>` inside of code, that means you replace it with your own custom content.
[^2]: You can get the commit id from `git log --oneline`. It should look something like this: `bd74f2c`.
[^3]: Remember that staged content should be green.
[^4]: `css/` should be red.
[^5]: Inside the main project directory.
[^6]: Remember that you have to be in your main project folder. If you are not, just navigate to it using the `cd` command.
[^7]: **Push**ing files means to adding them to the online repository.
[^8]: Alternatively, run `git checkout -b temp`. This creates a new branch called temp and automatically switches to it.