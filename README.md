# Tool for easily creating kysely migrations

### 1. Configuring User-Specific Bin Directory and Updating PATH

On Fedora, and most other Linux distributions, each user's local bin directory is typically found at `~/bin` or `~/.local/bin`. The tilde `~` represents the user's home directory.

To add this directory to your PATH (if it's not already there), you can add the following line to your shell's startup file (e.g., `~/.bashrc` for Bash or `~/.zshrc` for Zsh):

```sh
export PATH="$PATH:$HOME/bin:$HOME/.local/bin"
```

After adding this line, you will need to restart your terminal or run the `source` command on the startup file to apply the changes. For example:

```sh
source ~/.bashrc
```

Or for Zsh:

```sh
source ~/.zshrc
```

Compiling a CLI (Command-Line Interface) Rust app and making it globally available on a Linux OS involves several steps. Below is a guide to help you through the process.

### 2. Build the Release Version of Your App

Next, you should compile your app in release mode to optimize for performance:

```sh
cargo build --release
```

After running this command, you will find the compiled binary in `target/release/`.

### 3. Make the Binary Executable (if it’s not already)

Ensure that the binary is executable:

```sh
chmod +x target/release/branching
```

### 4. Move the Binary to a Directory in Your PATH

Linux systems have a set of directories where they look for executable files when you type a command. You need to move or copy your binary to one of these directories. A common choice for user-specific, manually installed binaries is `/usr/local/bin/`.

First, you might want to check that `/usr/local/bin` is in your PATH:

```sh
echo $PATH
```

You should see `/usr/local/bin` listed in the output. If not, you will need to add it to your PATH, or choose a different directory that is in your PATH.

Next, move the binary:

```sh
sudo mv target/release/branching /usr/local/bin/
```

Now, you should be able to run your program from anywhere in your system just by typing its name:

```sh
branching
```

### 5. (Optional) Set Permissions

If you want other users on your system to be able to run the app, you may need to set the appropriate permissions. For example:

```sh
sudo chmod 755 /usr/local/bin/branching
```

This command gives read and execute permissions to everyone and write permission to the owner.

That's it! Your Rust CLI app should now be globally available on your Linux OS.
