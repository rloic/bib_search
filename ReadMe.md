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
bib_search bib_files -q queries
```

The first arguments are the bib files, the `-q` arguments are the queries. Each query must be of the format `field:value`. There is several shortcuts for the fields:

| Shortcut | Equivalent Field |
| -------- | ---------------- |
| `t`      | title            |
| `y`      | year             |
| `a`      | author           |
| `c`      | citation key     |
| `p`      | publisher        |

By default the results will be displayed in the default `BibTex` format.

```sh
bib_search crypto.bib  -q 'c:C:FouJeaPey13' 'c:EC:BirNik10'
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
bib_search crypto.bib personnal.bib  -t -q 'c:C:FouJeaPey13' 'c:EC:BirNik10'
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

To simplify the search, I have tried to remove punctuations and the search in insensitive, for example a search value `to aes` will match `to {AES}`. To apply `AND` filters, which means both conditions must be required, you must use the `&` character.  For example, you may write the following query: `t:high&a:knu` which means that I wan the entries that contains `high` in their title field and `knu` in their author field. To have different filters you have just to separate the different queries by a space `bib_search crypto.bib personnal.bib  -t -q 'c:C:FouJeaPey13' 'c:EC:BirNik10'` means that I search entries which have the citation key `C:FouJeaPey13` or `EC:BirNik10`.

