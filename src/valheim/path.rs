use std::{borrow::Borrow, env, ffi::OsStr, fmt, marker::PhantomData, path::PathBuf};

#[derive(Debug)]
pub struct Path<T>(PathBuf, PhantomData<T>);

#[derive(Debug)]
pub enum Server {}
#[derive(Debug)]
pub enum Saves {}

impl Default for Path<Server> {
    fn default() -> Self {
        let program_files = env::var("ProgramFiles(x86)").expect("!ProgramFiles(x86)");
        let mut path = PathBuf::from(program_files);
        path.push(r"Steam\steamapps\common\Valheim dedicated server");

        Self(path, PhantomData)
    }
}

impl Default for Path<Saves> {
    fn default() -> Self {
        let mut local_appdata = env::var("LOCALAPPDATA").expect("!LOCALAPPDATA");
        local_appdata.push_str("Low");

        let mut path = PathBuf::from(local_appdata);
        path.push(r"IronGate\Valheim");

        Self(path, PhantomData)
    }
}

impl<T> fmt::Display for Path<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string_lossy().borrow())
    }
}

impl<T, P: AsRef<OsStr>> From<P> for Path<T> {
    fn from(path: P) -> Self {
        Self(PathBuf::from(path.as_ref()), PhantomData)
    }
}
