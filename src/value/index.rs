use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
enum EnumIndex {
    Key(String),
    Index(isize),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Index {
    part: EnumIndex,
}

impl Index {
    pub fn with_key<S: Into<String>>(key: S) -> Self {
        let part = EnumIndex::Key(key.into());
        Self { part }
    }

    pub fn with_index(index: isize) -> Self {
        let part = EnumIndex::Index(index);
        Self { part }
    }

    pub fn is_key(&self) -> bool {
        match self.part {
            EnumIndex::Key(_) => true,
            EnumIndex::Index(_) => false,
        }
    }

    pub fn is_index(&self) -> bool {
        match self.part {
            EnumIndex::Key(_) => false,
            EnumIndex::Index(_) => true,
        }
    }

    pub fn as_key(&self) -> Option<&str> {
        match self.part {
            EnumIndex::Key(ref k) => Some(k),
            EnumIndex::Index(_) => None,
        }
    }

    pub fn as_index(&self) -> Option<isize> {
        match self.part {
            EnumIndex::Key(_) => None,
            EnumIndex::Index(k) => Some(k),
        }
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.part {
            EnumIndex::Key(ref k) => write!(f, "{}", k),
            EnumIndex::Index(k) => write!(f, "{}", k),
        }
    }
}

impl From<String> for Index {
    fn from(k: String) -> Self {
        Self::with_key(k)
    }
}

impl<'a> From<&'a str> for Index {
    fn from(k: &'a str) -> Self {
        Self::with_key(k)
    }
}
