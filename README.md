## Use LLMs to analyse files through CLI!

This project implements a simple CLI tool with Rust that leverages local LLM models 
through Ollama to analyse texts of different format files.

Currently, only the following file types are supported:
* PDF
* DOC
* HTML
* TXT
* MARKDOWN

### Requirements (outside of Cargo.toml)

* Ollama (note: you need to pull your local model first before using the CLI tool)
* pdftohtml
* soffice
* Obsidian with an API plugin (if you want to save your LLM outputs to Obsidian)

You can easily install all of these using `brew`.

### Installation & Configuration

To install this on your machine, just run:

```
git clone https://github.com/MattG-bci/rust-cli
```

Now, after you clone the repository and install all the requirements, you need to
configure the tool to your needs. Luckily, it is as simple as running a single bash
script in the `scripts/` directory. You can do this by:

```
<your_absolute_path_to_repository>/scripts/configure.sh
```

This script will ask you about:

* Whether you want your outputs to be saved locally or your Obsidian vault.
* Couple parameters that are needed to process and save LLM responses (such as Obsidian API key, out_dir if outputs is meant to be saved locally)

After doing that, two aliases will be added to your .zshrc file so that you will be able to trigger this tool from anywhere.
These are:

````
rust-llm-configure --> if you would like to reconfigure your parameters
rust-llm {*ARGS} --> actual command for the CLI tool
````

To use the latter command, you need to specify (in that order):
* command: summarise or explain (for now only these options are available)
* model: local model you want to use (you need to pull it with Ollama first)
* with_thinking: whether to use thinking mode of your LLM (False as a default)
* path_to_file: path to the file you want to analyse

Aaaaand... that's it! Enjoy!
