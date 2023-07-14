# MiniGrep

## About

A command line program similar to `grep`/`ripgrep`. Taught as a mini project in [chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) of the [official rust book](https://doc.rust-lang.org/book/).

Basically it lets you search for all lines within a text file
that contain a particular string(query).

## Steps to set up and run

1. [Set up rust on your system](https://www.rust-lang.org/tools/install)

2. Clone this repository:-

   ```
   git clone https://github.com/xoldyckk/minigrep.git
   ```

3. Change directory of your terminal into the cloned project folder.

   ```
   cd ./minigrep
   ```

4. Run the following command:-

   ```
   cargo run -- <query> <path_to_text_file>
   ```

   where `<query>` is the string which you want to search for and `<path_to_text_file>` is path to the text file you
   want to search the `query` in for.

   Optionally, if you want the search to be case insensitive provide an environment variable named `IGNORE_CASE` like this:-

   ```
   IGNORE_CASE=1 cargo run --<query> <path_to_text_file>
   ```
