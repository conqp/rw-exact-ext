# rw-exact-ext
Extension of `std::io` to read and write data types with exact amounts of bytes.

## Usage
This library provides two traits, `rw_exact_ext::ReadExactExt` and `rw_exact_ext::WriteAllExt`.

### `ReadExactExt`
This trait provides functions to read byte arrays of a constant size 
and vectors of a runtime-defined size from a reader that implements `std::io::Read`.  
If the feature `num-traits` is enabled, it also provides functions to read numbers from such a reader.

### `WriteAllExt`
This trait provides writers that implement `std::io::Write` with an additional method to write booleans.
If the feature `num-traits` is enabled, it also provides functions to write numbers to such a writer.
