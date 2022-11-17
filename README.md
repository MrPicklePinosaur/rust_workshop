
<div align="center">

<div>
  <img src="dist/csc.svg" width="20%" height="auto"/>
  <img src="dist/ferris.png" width="15%" height="auto"/>
</div>

# rust_workshop

rust workshop to build an argument parser and cli for CSC Project Program

</div>

## Prerequisites

First install **rust** using **rustup** (https://rustup.rs):

For linux, macos and WSL users, we can just run the command:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For windows users (without WSL), you can download and run `rustup-init.exe`
from the **rustup** website and install dependencies using Visual Studio.

After **rustup** is finished, make sure to restart your shell or reload your
PATH environment variable. You can double check that everything install
correctly by running:
```
cargo --version
```

We also need **cargo-script** to be able to run our example code. You
can install it with:
```
cargo install cargo-script
```

And you can clone this repository to have access to the slides and example code.
```
git clone https://github.com/MrPicklePinosaur/rust_workshop
```

## Running example code

The code for each slide can be found in the `code/` directory. We can run it using **cargo-script**.
```
cargo script code/1.rs
```

If **cargo-script** for some reason was not working for you, you can also just compile and execute the code normally
```
rustc code/1.rs
./1 a b c
```

