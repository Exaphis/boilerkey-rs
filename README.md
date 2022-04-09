# boilerkey-rs

A reimplementation of [elnardu/local-boilerkey](https://github.com/elnardu/local-boilerkey)
for fun in Rust.

## Build

Run `cargo build`.

## Usage

1. Run the built binary.
2. Add a new Duo Mobile BoilerKey from the
   [BoilerKey Self-Serve](https://www.purdue.edu/apps/account/flows/BoilerKey) page.
3. Enter the activation code. This is the sequence of letters and numbers in the
   "Visit this URL with your smartphone" section.
    * When the URL is `https://m-1b9bef70.duosecurity.com/activate/ABCXYZ`, the
      activation code is ABCXYZ.
4. Enter your BoilerKey PIN.
5. Done! Every time you run `boilerkey-rs`, a new code will be generated in `pin,code`
   format.
    * Data for `boilerkey-rs` is stored in `hotp_data.json` in the working directory.

```console
$ target/release/boilerkey-rs
Enter activation code: ABCXYZ
Requesting activation data...
Response: <snip>
Enter BoilerKey PIN: pin
pin,123456
```
