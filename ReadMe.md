# BibSearch

This tool aims to search bib entries in large bib files.

## Installation

The installation step requires to have the rust compiler installed (see [here](https://www.rust-lang.org/learn/get-started)).

Once `rust` is installed you can build the project from `git` with :

```sh
git clone https://github.com/rloic/bib_search
cd bib_search
cargo build --release
```

or directly with `cargo`:

```sh
cargo install --git https://github.com/rloic/bib_search
```

## Usage

The installation command will generate the executable under the `target/release` folder.

You can run the executable with :

```sh
bib_search bib_files -q query
```

By default the results will be displayed in the default `BibTex` format.

```sh
bib_search crypto.bib  -q "cite_key = 'C:FouJeaPey13' or cite_key = 'EC:BirNik10'"
```

```bibtex
@InProceedings{EC:BirNik10,
  address = eurocrypt10addr,
  author = "Alex Biryukov and Ivica Nikolic",
  booktitle = eurocrypt10name,
  doi = "10.1007/978-3-642-13190-5_17",
  editor = eurocrypt10ed,
  month = eurocrypt10month,
  pages = "322--344",
  publisher = eurocryptpub,
  series = mylncs,
  title = "Automatic Search for Related-Key Differential Characteristics in Byte-Oriented Block Ciphers: Application to {AES}, {Camellia}, {Khazad} and Others",
  volume = eurocrypt10vol,
  year = 2010,
}
@InProceedings{C:FouJeaPey13,
  address = cryptoaddr,
  author = "Pierre-Alain Fouque and J{\'e}r{\'e}my Jean and Thomas Peyrin",
  booktitle = crypto13name1,
  doi = "10.1007/978-3-642-40041-4_11",
  editor = crypto13ed,
  month = crypto13month,
  pages = "183--203",
  publisher = cryptopub,
  series = mylncs,
  title = "Structural Evaluation of {AES} and Chosen-Key Distinguisher of 9-Round {AES}-128",
  volume = crypto13vol1,
  year = 2013,
}
```

You can also display in tab format with the `-t` flag which display the entry type, the citation key, the title, the authors and the year.

```sh
bib_search crypto.bib personnal.bib  -t -q "cite_key = 'C:FouJeaPey13' or cite_key = 'EC:BirNik10'"
```

```text
+---------------+---------------+------------------------------------------+---------------------+------+
+ Type          + Cite key      + Title                                    + Author(s)           + Year +
+---------------+---------------+------------------------------------------+---------------------+------+
| InProceedings | EC:BirNik10   | Automatic Search for Related-Key         | Alex Biryukov       | 2010 |
|               |               | Differential Characteristics in          | Ivica Nikolic       |      |
|               |               | Byte-Oriented Block Ciphers:             |                     |      |
|               |               | Application to {AES}, {Camellia},        |                     |      |
|               |               | {Khazad} and Others                      |                     |      |
+---------------+---------------+------------------------------------------+---------------------+------+
| InProceedings | C:FouJeaPey13 | Structural Evaluation of {AES} and       | Pierre-Alain Fouque | 2013 |
|               |               | Chosen-Key Distinguisher of 9-Round      | J{\'e}r{\'e}my Jean |      |
|               |               | {AES}-128                                | Thomas Peyrin       |      |
+---------------+---------------+------------------------------------------+---------------------+------+
```

## Search rules

The search engines use SQL syntax but supports a limited set of operators. The current implemented operators are: `=`, `!=`, `>`, `>=`, `<`, `<=`, `like`, `ilike`, `and` and `or`.

