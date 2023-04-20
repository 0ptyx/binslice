mod binslice {
    use std::fmt::{self, Display, Formatter};
    use std::io;
    use std::path::Path;
    use std::rc::Rc;

    // Want to devise a trait for T to constrain
    // what it could be -- something meaningful to
    // being able to spit back the cleaned up binary
    // data, reach into its fields, and potentially
    // re-store it as a C-FFI binary data dump
    pub(crate) struct BinSlice<T>
    where
        T: Entry + Display + Sized,
    {
        data: Box<Vec<u8>>,
        end: Endianness,
        info: Option<Details<T>>,
    }

    pub(crate) enum Endianness {
        Big,
        Little,
        Mixed,
        Unknown,
    }
    pub(crate) struct Details<E: Entry + Display + Sized> {
        title: String,
        entries: Box<Vec<Rc<E>>>,
    }

    impl<E> Details<E>
    where
        E: Entry + Display + Sized,
    {
        fn print(&self, dest: impl AsRef<Path>) -> io::Result<()> {
            todo!("Need to flesh this out");
        }

        // Needs to acquire exclusive write access to STDERR
        fn to_stderr(&self) -> io::Result<()> {
            todo!("Need to flesh this out");
        }
    }

    pub(crate) trait Entry {}

    impl<E> fmt::Display for Details<E>
    where
        E: Entry + Display + Sized,
    {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{}\n", self.title);
            for entry in self.entries.clone().into_iter() {
                write!(f, "{}", &entry);
            }
            fmt::Result::Ok(())
        }
    }
}
pub(crate) use binslice::{BinSlice, Details, Endianness, Entry};
