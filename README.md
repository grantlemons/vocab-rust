# Vocab Completion

This program takes in a list of words in spanish and uses [wordreference.com](wordreference.com) to fetch information about these words such as definition etc.
The program then compiles these into a table laid out in the same way as our monthly vocabulary.
With this tool, the tedious parts of spanish vocab word are eliminated, and the only burden for you is to make a list of words and to write the example sentences.

An example of the program's use is below.

## Example Usage

```text
Enter an empty line to continue.
===============
Word 1: delantal
Word 2: madrugar    
Word 3: 
... 
```

Once completed, a dialog will appear for each word that has multiple definitions on wordreference.
Each option will have an index number next to it and list information about the option such as part of speech, an example in english and spanish, etc.
Enter the desired index and press `Enter`.
The dialog will appear once more if other words require a selection.

```text
Definition options for delantal
========================================================================
1:  delantal             (nm)       mandil                                   ["El delantal del chef estaba sucio por haber trabajado en la elaboración de tan exquisita cena."] / ["The chef's apron was dirty from working on such an exquisite dinner."]
2:  delantal             (nf)       prenda de protección                     ["El nuevo soldador necesita un delantal para ponerse a trabajar."] / ["The new welder needs an apron so he can get to work."]
3:  delantal             (nf)       prenda de protección                     ["El nuevo soldador necesita un delantal para ponerse a trabajar."] / ["The new welder needs an apron so he can get to work."]
Index: 
```

Once all words are inputted, a table will be printed like so.

```text
|    Palabra                      |     Categoría      |       Fuente       |        Definición y diccionario        |      Contexto      |
|=================================|====================|====================|========================================|====================|
|    silbar                       |        vtr         |                    |              chiflar (WR)              |                    |
|    herida                       |         nf         |                    |      sufrimiento, aflicción (WR)       |                    |
|    delantal                     |         nm         |                    |              mandil (WR)               |                    |
|    horario                      |         nm         |                    |           programación (WR)            |                    |
|    guía                         |      n común       |                    |      quien conduce a alguien (WR)      |                    |
|    cola                         |         nf         |                    |           animal: rabo (WR)            |                    |
|    primordialmente              |        adv         |                    |        de manera esencial (WR)         |                    |
|    arena                        |         nf         |                    |         roca pulverizada (WR)          |                    |
|    inútil                       |       adj mf       |                    |          persona: inepta (WR)          |                    |
|    frívolo                      |        adj         |                    |            superficial (WR)            |                    |
|    engreído                     |        adj         |                    |             vanidoso (WR)              |                    |
|    alimento                     |         nm         |                    |         comida, sustento (WR)          |                    |
|    condescendiente              |        adj         |                    |         desdeñoso, altivo (WR)         |                    |
|    burgués                      |       nm, nf       |                    |    persona de clase media alta (WR)    |                    |
|    proletariado                 |         nm         |                    |       clase social: obrera (WR)        |                    |
|    puntual                      |        adj         |                    |        que llega a tiempo (WR)         |                    |
|    mármol                       |         nm         |                    |           piedra caliza (WR)           |                    |
|    político                     |        adj         |                    |    que se dedica a la política (WR)    |                    |
|    ladrillo                     |         nm         |                    |     material de construcción (WR)      |                    |
|    yeso                         |         nm         |                    |             material (WR)              |                    |
```

## Compilation

### Installing Rust

If not already installed, install Rust for your operating system and architecture.
In all likelihood this is Windows and x86_64, but may be something else such as an arm-based architecture.

Download and use the installation binary [here](https://forge.rust-lang.org/infra/other-installation-methods.html#standalone). In you are on x86_64 and Windows, this will be the [`x86_64-pc-windows-msvc` stable `msi` file.](https://static.rust-lang.org/dist/rust-1.67.0-x86_64-pc-windows-msvc.msi)

## Compiling

- Clone this project into a folder.
- Use the command line to run `cargo build --release`

## Executing

If the command above is used, the executable will be located at `target/release/vocab-completer.exe`.
Otherwise, the program can be run using `cargo run --release` in the project's directory.
