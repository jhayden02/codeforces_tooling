# codeforces_tooling

A simple Codeforces work repository to help you do the following:
- Generate solution files from a url of the problem using templates
- Run the solution files using their specified build configurations with 1 language-agnostic script

## Setup

```sh
carton install
```

## Usage

Create a solution file from a template in `problemset/`:

```sh
./new <problem_url> <c|cpp|py|rs>
```

Compile and run a solution:

```sh
./run <file>
```
