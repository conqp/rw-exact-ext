use std::io::{Read, Result, Write};

pub trait ReadExactExt: Read {
    /// Read a byte array of a constant size.
    ///
    /// For further semantics please refer to [`Read::read_exact`].
    ///
    /// # Examples
    /// ```
    /// use rw_exact_ext::ReadExactExt;
    /// use std::io::Cursor;
    ///
    /// let bytes = [0xAB, 0xCD, 0xEF, 0x42];
    /// let array: [u8; 4] = Cursor::new(&bytes).read_array_exact().unwrap();
    /// assert_eq!(array, bytes);
    /// ```
    #[allow(clippy::missing_errors_doc)]
    fn read_array_exact<const SIZE: usize>(&mut self) -> Result<[u8; SIZE]> {
        let mut buffer = [0; SIZE];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    /// Read one byte and interpret it as a `bool`.
    ///
    /// Returns `true` if the read byte is non-zero, or `false` otherwise.
    ///
    /// For further semantics please refer to [`Read::read_exact`].
    ///
    /// # Examples
    /// ```
    /// use rw_exact_ext::ReadExactExt;
    /// use std::io::Cursor;
    ///
    /// let bytes = [0x01, 0x00, 0xEF, 0x42];
    /// let mut cursor = Cursor::new(&bytes);
    /// assert!(cursor.read_bool().unwrap());
    /// assert!(!cursor.read_bool().unwrap());
    /// assert!(cursor.read_bool().unwrap());
    /// assert!(cursor.read_bool().unwrap());
    /// ```
    #[allow(clippy::missing_errors_doc)]
    fn read_bool(&mut self) -> Result<bool> {
        self.read_array_exact::<1>().map(|[byte]| byte != 0)
    }

    /// Read a `Vec<u8>` of a given size.
    ///
    /// For further semantics please refer to [`Read::read_exact`].
    ///
    /// # Examples
    /// ```
    /// use rw_exact_ext::ReadExactExt;
    /// use std::io::Cursor;
    ///
    /// let bytes = [0xAB, 0xCD, 0xEF, 0x42];
    /// let vec = Cursor::new(&bytes).read_vec_exact(bytes.len()).unwrap();
    /// assert_eq!(vec, Vec::from(bytes));
    /// ```
    #[allow(clippy::missing_errors_doc)]
    fn read_vec_exact(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0; size];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    /// Read a number from a byte array in big endian.
    ///
    /// For further semantics please refer to [`Read::read_exact`].
    ///
    /// # Examples
    /// ```
    /// use rw_exact_ext::ReadExactExt;
    /// use std::io::Cursor;
    ///
    /// let bytes = [0xAB, 0xCD, 0xEF, 0x42];
    ///
    /// let unsigned: u32 = Cursor::new(&bytes).read_num_be().unwrap();
    /// assert_eq!(unsigned, 0xABCDEF42);
    ///
    /// let signed: i32 = Cursor::new(&bytes).read_num_be().unwrap();
    /// assert_eq!(signed, -0x543210BE);
    ///
    /// let float: f32 = Cursor::new(&bytes).read_num_be().unwrap();
    /// assert_eq!(float, -1.4632533e-12);
    /// ```
    #[cfg(feature = "num-traits")]
    #[allow(clippy::missing_errors_doc)]
    fn read_num_be<N, const SIZE: usize>(&mut self) -> Result<N>
    where
        N: num_traits::FromBytes<Bytes = [u8; SIZE]>,
    {
        self.read_array_exact()
            .map(|bytes| N::from_be_bytes(&bytes))
    }

    /// Read a number from a byte array in little endian.
    ///
    /// For further semantics please refer to [`Read::read_exact`].
    ///
    /// # Examples
    /// ```
    /// use rw_exact_ext::ReadExactExt;
    /// use std::io::Cursor;
    ///
    /// let bytes = [0xAB, 0xCD, 0xEF, 0x42];
    ///
    /// let unsigned: u32 = Cursor::new(&bytes).read_num_le().unwrap();
    /// assert_eq!(unsigned, 0x42EFCDAB);
    ///
    /// let signed: i32 = Cursor::new(&bytes).read_num_le().unwrap();
    /// assert_eq!(signed, 0x42EFCDAB);
    ///
    /// let float: f32 = Cursor::new(&bytes).read_num_le().unwrap();
    /// assert_eq!(float, 119.901695);
    /// ```
    #[cfg(feature = "num-traits")]
    #[allow(clippy::missing_errors_doc)]
    fn read_num_le<N, const SIZE: usize>(&mut self) -> Result<N>
    where
        N: num_traits::FromBytes<Bytes = [u8; SIZE]>,
    {
        self.read_array_exact()
            .map(|bytes| N::from_le_bytes(&bytes))
    }

    /// Read a number from a byte array in native endianness.
    ///
    /// For further semantics please refer to [`Read::read_exact`].
    #[cfg(feature = "num-traits")]
    #[allow(clippy::missing_errors_doc)]
    fn read_num_ne<N, const SIZE: usize>(&mut self) -> Result<N>
    where
        N: num_traits::FromBytes<Bytes = [u8; SIZE]>,
    {
        self.read_array_exact()
            .map(|bytes| N::from_ne_bytes(&bytes))
    }
}

impl<T> ReadExactExt for T where T: Read {}

pub trait WriteAllExt: Write {
    /// Write a number to bytes in big endian.
    ///
    /// For further semantics please refer to [`Write::write_all`].
    ///
    /// # Examples
    /// ```
    /// use rw_exact_ext::WriteAllExt;
    /// use std::io::Cursor;
    ///
    /// let mut bytes = vec![0u8; 4];
    ///
    /// let unsigned: u32 = 1337;
    /// Cursor::new(&mut bytes).write_num_be(unsigned).unwrap();
    /// assert_eq!(bytes, vec![0x00, 0x00, 0x05, 0x39]);
    ///
    /// let signed: i32 = -1337;
    /// Cursor::new(&mut bytes).write_num_be(signed).unwrap();
    /// assert_eq!(bytes, vec![0xFF, 0xFF, 0xFA, 0xC7]);
    ///
    /// let float: f32 = 133.7;
    /// Cursor::new(&mut bytes).write_num_be(float).unwrap();
    /// assert_eq!(bytes, vec![0x43, 0x05, 0xB3, 0x33]);
    /// ```
    #[cfg(feature = "num-traits")]
    #[allow(clippy::missing_errors_doc)]
    fn write_num_be<N, const SIZE: usize>(&mut self, num: N) -> Result<()>
    where
        N: num_traits::ToBytes<Bytes = [u8; SIZE]>,
    {
        self.write_all(&num.to_be_bytes())
    }

    /// Write a number to bytes in little endian.
    ///
    /// For further semantics please refer to [`Write::write_all`].
    ///
    /// # Examples
    /// ```
    /// use rw_exact_ext::WriteAllExt;
    /// use std::io::Cursor;
    ///
    /// let mut bytes = vec![0u8; 4];
    ///
    /// let unsigned: u32 = 1337;
    /// Cursor::new(&mut bytes).write_num_le(unsigned).unwrap();
    /// assert_eq!(bytes, vec![0x39, 0x05, 0x00, 0x00]);
    ///
    /// let signed: i32 = -1337;
    /// Cursor::new(&mut bytes).write_num_le(signed).unwrap();
    /// assert_eq!(bytes, vec![0xC7, 0xFA, 0xFF, 0xFF]);
    ///
    /// let float: f32 = 133.7;
    /// Cursor::new(&mut bytes).write_num_le(float).unwrap();
    /// assert_eq!(bytes, vec![0x33, 0xB3, 0x05, 0x43]);
    /// ```
    #[cfg(feature = "num-traits")]
    #[allow(clippy::missing_errors_doc)]
    fn write_num_le<N, const SIZE: usize>(&mut self, num: N) -> Result<()>
    where
        N: num_traits::ToBytes<Bytes = [u8; SIZE]>,
    {
        self.write_all(&num.to_le_bytes())
    }

    /// Write a number to bytes in native endianness.
    ///
    /// For further semantics please refer to [`Write::write_all`].
    #[cfg(feature = "num-traits")]
    #[allow(clippy::missing_errors_doc)]
    fn write_num_ne<N, const SIZE: usize>(&mut self, num: N) -> Result<()>
    where
        N: num_traits::ToBytes<Bytes = [u8; SIZE]>,
    {
        self.write_all(&num.to_ne_bytes())
    }
}

impl<T> WriteAllExt for T where T: Write {}
