//! The trash directory itself: putting entries in, listing them, restoring
//! them.

use crate::error::Error;
use crate::trashinfo::TrashInfo;
use std::ffi::{OsStr, OsString};
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// A trash directory laid out as the freedesktop.org Trash spec describes:
///
/// ```text
/// <root>/
/// ├── files/   the trashed entries themselves
/// └── info/    one <name>.trashinfo per entry
/// ```
#[derive(Debug)]
pub struct Trash {
    root: PathBuf,
}

/// One entry in the trash.
#[derive(Debug, PartialEq, Eq)]
pub struct TrashEntry {
    /// The entry's name inside `files/`. Usually the original file name, but
    /// with `_1`, `_2`, … appended if that was already taken. This is what
    /// `akuta --restore` expects.
    pub name: OsString,

    /// What its `.trashinfo` says.
    pub info: TrashInfo,
}

impl Trash {
    /// A trash rooted at `root`. Nothing is touched on disk until a method
    /// needs it.
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// The trash's top-level directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// `<root>/files`, where trashed entries are kept.
    pub fn files_dir(&self) -> PathBuf {
        self.root.join("files")
    }

    /// `<root>/info`, where the `.trashinfo` files are kept.
    pub fn info_dir(&self) -> PathBuf {
        self.root.join("info")
    }

    /// `<root>/info/<name>.trashinfo`, the info file for the entry `name`.
    pub fn info_path(&self, name: &OsStr) -> PathBuf {
        let mut file_name = name.to_os_string();
        file_name.push(".trashinfo");
        self.info_dir().join(file_name)
    }

    /// Creates the root, `files/` and `info/` directories if they do not
    /// exist yet. Directories this creates get mode `0700` (owner only), as
    /// the Trash spec requires; existing ones are left as they are.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Io`] if a directory cannot be created.
    pub fn ensure_dirs(&self) -> Result<(), Error> {
        todo!("create the three directories with mode 0700 (see DirBuilderExt)")
    }

    /// Moves `target` into the trash and returns the name it was given.
    ///
    /// `target` must be an absolute path, as returned by
    /// [`resolve_target`](crate::safety::resolve_target). The trash
    /// directories must already exist (see [`Trash::ensure_dirs`]).
    ///
    /// The order of the steps matters; see "Deletion flow" in issue #41:
    ///
    /// 1. Pick a name: try `numbered(file_name, 0)`, then `1`, `2`, … (see
    ///    [`numbered`](crate::naming::numbered)). A name is taken if
    ///    `files/<name>` exists (checked with `symlink_metadata`, so broken
    ///    symlinks count) or if creating `info/<name>.trashinfo` with
    ///    `OpenOptions::create_new` fails with `AlreadyExists`. Creating the
    ///    info file *is* the reservation: `create_new` is atomic, so two
    ///    runs at once can never claim the same name.
    /// 2. Write the [`TrashInfo`] into that file, dated with `deleted_at`.
    /// 3. `std::fs::rename` `target` to `files/<name>`.
    /// 4. If the rename fails, remove the info file again and return the
    ///    rename's error.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Io`] if any filesystem step fails, including the
    /// rename (e.g. `target` is on another device), and
    /// [`Error::SystemTime`] if `deleted_at` is before the Unix epoch.
    #[expect(unused_variables, reason = "body is todo!(); delete once implemented")]
    pub fn put(&self, target: &Path, deleted_at: SystemTime) -> Result<OsString, Error> {
        todo!("reserve a name via the .trashinfo file, then rename target into files/")
    }

    /// Lists every entry in the trash, newest first (by `DeletionDate`).
    ///
    /// Every `*.trashinfo` file in `info/` is one entry, named by the file
    /// name without its `.trashinfo` suffix. Other files in `info/` are
    /// ignored. A trash whose `info/` directory does not exist yet is simply
    /// empty.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Io`] if `info/` or a file in it cannot be read, and
    /// [`Error::InvalidTrashInfo`] if any `.trashinfo` file is malformed.
    pub fn list(&self) -> Result<Vec<TrashEntry>, Error> {
        todo!("read info/, parse each .trashinfo, sort newest first")
    }

    /// Moves the entry `name` back to where it came from, and returns the
    /// path it was restored to.
    ///
    /// If something already exists at the original path (checked with
    /// `symlink_metadata`), asks for a new name instead of overwriting it:
    ///
    /// 1. The suggestion is the first `numbered(file_name, n)` for `n >= 1`
    ///    that does not exist in the same directory (`a.txt_1`, `a.txt_2`,
    ///    …).
    /// 2. Write the prompt to `output`, e.g.
    ///    `'/home/me/a.txt' already exists. Restore as [a.txt_1]: `, and
    ///    read one line from `input`.
    /// 3. An empty answer accepts the suggestion; anything else is a file
    ///    name in the same directory. If that is taken too, ask again.
    ///
    /// Once the destination is settled, `rename` `files/<name>` there, then
    /// remove `info/<name>.trashinfo`.
    ///
    /// `input` and `output` are stdin and stdout in `main`; taking them as
    /// parameters is what lets the tests answer the prompt.
    ///
    /// # Errors
    ///
    /// - [`Error::NotInTrash`] if there is no entry called `name`.
    /// - [`Error::RestoreDirMissing`] if the original directory is gone.
    /// - [`Error::Aborted`] if `input` ends before a usable answer.
    /// - [`Error::Io`] or [`Error::InvalidTrashInfo`] if reading the entry
    ///   or moving it fails.
    #[expect(unused_variables, reason = "body is todo!(); delete once implemented")]
    pub fn restore(
        &self,
        name: &OsStr,
        input: &mut impl BufRead,
        output: &mut impl Write,
    ) -> Result<PathBuf, Error> {
        todo!("read the .trashinfo, settle the destination, rename the file back")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::temp_dir;
    use std::fs;
    use std::io::Cursor;
    use std::os::unix::fs::{PermissionsExt, symlink};
    use std::time::{Duration, UNIX_EPOCH};

    /// `2026-09-24T22:00:00` UTC.
    fn sample_time() -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(1_790_287_200)
    }

    /// A trash under `dir`, with its directories created by hand so that
    /// these tests do not depend on `ensure_dirs`.
    fn trash_in(dir: &Path) -> Trash {
        let trash = Trash::new(dir.join("Trash"));
        fs::create_dir_all(trash.files_dir()).unwrap();
        fs::create_dir_all(trash.info_dir()).unwrap();
        trash
    }

    /// Puts an entry into `trash` by hand, without going through `put`:
    /// a file `files/<name>` containing `name`, and its `.trashinfo`.
    fn add_entry(trash: &Trash, name: &str, original: &Path, date: &str) {
        fs::write(trash.files_dir().join(name), name).unwrap();
        fs::write(
            trash.info_path(OsStr::new(name)),
            format!(
                "[Trash Info]\nPath={}\nDeletionDate={date}\n",
                original.display()
            ),
        )
        .unwrap();
    }

    fn mode_of(path: &Path) -> u32 {
        fs::metadata(path).unwrap().permissions().mode() & 0o777
    }

    // info_path

    #[test]
    fn info_path_appends_the_trashinfo_extension() {
        let trash = Trash::new(PathBuf::from("/t"));

        assert_eq!(
            trash.info_path(OsStr::new("a.txt")),
            PathBuf::from("/t/info/a.txt.trashinfo")
        );
    }

    // ensure_dirs

    #[test]
    fn ensure_dirs_creates_private_directories() {
        let dir = temp_dir("ensure-dirs");
        let trash = Trash::new(dir.join("Trash"));

        trash.ensure_dirs().unwrap();

        assert_eq!(mode_of(&trash.files_dir()), 0o700);
        assert_eq!(mode_of(&trash.info_dir()), 0o700);

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn ensure_dirs_twice_is_fine() {
        let dir = temp_dir("ensure-dirs-twice");
        let trash = Trash::new(dir.join("Trash"));

        trash.ensure_dirs().unwrap();
        trash.ensure_dirs().unwrap();

        fs::remove_dir_all(&dir).unwrap();
    }

    // put

    #[test]
    fn put_moves_a_file_into_the_trash() {
        let dir = temp_dir("put-file");
        let trash = trash_in(&dir);
        let target = dir.join("a.txt");
        fs::write(&target, "hello").unwrap();

        let name = trash.put(&target, sample_time()).unwrap();

        assert_eq!(name, "a.txt");
        assert!(!target.exists());
        assert_eq!(
            fs::read_to_string(trash.files_dir().join("a.txt")).unwrap(),
            "hello"
        );

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn put_writes_the_trashinfo() {
        let dir = temp_dir("put-info");
        let trash = trash_in(&dir);
        let target = dir.join("a.txt");
        fs::write(&target, "").unwrap();

        trash.put(&target, sample_time()).unwrap();

        assert_eq!(
            fs::read_to_string(trash.info_path(OsStr::new("a.txt"))).unwrap(),
            format!(
                "[Trash Info]\nPath={}\nDeletionDate=2026-09-24T22:00:00\n",
                target.display()
            )
        );

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn put_moves_a_directory_with_its_contents() {
        let dir = temp_dir("put-dir");
        let trash = trash_in(&dir);
        let target = dir.join("sub");
        fs::create_dir(&target).unwrap();
        fs::write(target.join("inner.txt"), "x").unwrap();

        trash.put(&target, sample_time()).unwrap();

        assert!(!target.exists());
        assert!(trash.files_dir().join("sub").join("inner.txt").exists());

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn put_moves_a_broken_symlink_itself() {
        let dir = temp_dir("put-broken-symlink");
        let trash = trash_in(&dir);
        let target = dir.join("broken");
        symlink(dir.join("nowhere"), &target).unwrap();

        trash.put(&target, sample_time()).unwrap();

        let moved = fs::symlink_metadata(trash.files_dir().join("broken")).unwrap();
        assert!(moved.file_type().is_symlink());
        assert!(fs::symlink_metadata(&target).is_err());

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn put_numbers_a_name_already_in_the_trash() {
        let dir = temp_dir("put-collision");
        let trash = trash_in(&dir);
        fs::create_dir(dir.join("one")).unwrap();
        fs::create_dir(dir.join("two")).unwrap();
        fs::write(dir.join("one").join("a.txt"), "first").unwrap();
        fs::write(dir.join("two").join("a.txt"), "second").unwrap();

        let first = trash
            .put(&dir.join("one").join("a.txt"), sample_time())
            .unwrap();
        let second = trash
            .put(&dir.join("two").join("a.txt"), sample_time())
            .unwrap();

        assert_eq!(first, "a.txt");
        assert_eq!(second, "a.txt_1");
        assert_eq!(
            fs::read_to_string(trash.files_dir().join("a.txt_1")).unwrap(),
            "second"
        );

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn put_treats_a_name_taken_in_files_as_taken() {
        let dir = temp_dir("put-taken-in-files");
        let trash = trash_in(&dir);
        fs::write(trash.files_dir().join("a.txt"), "stray").unwrap();
        let target = dir.join("a.txt");
        fs::write(&target, "").unwrap();

        let name = trash.put(&target, sample_time()).unwrap();

        assert_eq!(name, "a.txt_1");
        assert_eq!(
            fs::read_to_string(trash.files_dir().join("a.txt")).unwrap(),
            "stray"
        );

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn put_treats_a_name_taken_in_info_as_taken() {
        let dir = temp_dir("put-taken-in-info");
        let trash = trash_in(&dir);
        fs::write(trash.info_path(OsStr::new("a.txt")), "stray").unwrap();
        let target = dir.join("a.txt");
        fs::write(&target, "").unwrap();

        let name = trash.put(&target, sample_time()).unwrap();

        assert_eq!(name, "a.txt_1");
        assert_eq!(
            fs::read_to_string(trash.info_path(OsStr::new("a.txt"))).unwrap(),
            "stray"
        );

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn put_removes_the_trashinfo_again_when_the_move_fails() {
        let dir = temp_dir("put-rollback");
        let trash = trash_in(&dir);
        // Never created, so the rename in step 3 fails after step 2 has
        // already written the .trashinfo.
        let target = dir.join("missing.txt");

        let result = trash.put(&target, sample_time());

        assert!(matches!(result, Err(Error::Io(_))));
        assert!(!trash.info_path(OsStr::new("missing.txt")).exists());

        fs::remove_dir_all(&dir).unwrap();
    }

    // list

    #[test]
    fn list_of_a_trash_that_does_not_exist_is_empty() {
        let dir = temp_dir("list-missing");
        let trash = Trash::new(dir.join("Trash"));

        assert_eq!(trash.list().unwrap(), Vec::new());

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn list_is_newest_first() {
        let dir = temp_dir("list-order");
        let trash = trash_in(&dir);
        add_entry(&trash, "mid", Path::new("/x/mid"), "2026-05-01T00:00:00");
        add_entry(&trash, "new", Path::new("/x/new"), "2026-09-24T22:00:00");
        add_entry(&trash, "old", Path::new("/x/old"), "2025-12-31T23:59:59");

        let names: Vec<OsString> = trash
            .list()
            .unwrap()
            .into_iter()
            .map(|entry| entry.name)
            .collect();

        assert_eq!(names, vec!["new", "mid", "old"]);

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn list_reads_each_trashinfo() {
        let dir = temp_dir("list-info");
        let trash = trash_in(&dir);
        add_entry(
            &trash,
            "a.txt_1",
            Path::new("/x/a.txt"),
            "2026-09-24T22:00:00",
        );

        assert_eq!(
            trash.list().unwrap(),
            vec![TrashEntry {
                name: OsString::from("a.txt_1"),
                info: TrashInfo {
                    original_path: PathBuf::from("/x/a.txt"),
                    deletion_date: "2026-09-24T22:00:00".to_string(),
                },
            }]
        );

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn list_ignores_files_that_are_not_trashinfo() {
        let dir = temp_dir("list-ignore");
        let trash = trash_in(&dir);
        add_entry(
            &trash,
            "a.txt",
            Path::new("/x/a.txt"),
            "2026-09-24T22:00:00",
        );
        fs::write(trash.info_dir().join("notes.txt"), "not an entry").unwrap();

        assert_eq!(trash.list().unwrap().len(), 1);

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn list_reports_a_malformed_trashinfo() {
        let dir = temp_dir("list-malformed");
        let trash = trash_in(&dir);
        fs::write(trash.info_path(OsStr::new("bad")), "garbage").unwrap();

        assert!(matches!(trash.list(), Err(Error::InvalidTrashInfo(_))));

        fs::remove_dir_all(&dir).unwrap();
    }

    // restore

    /// Restores `name`, answering the prompt (if any) with `answers`, and
    /// returns the result along with everything written to the output.
    fn restore_with(trash: &Trash, name: &str, answers: &str) -> (Result<PathBuf, Error>, String) {
        let mut input = Cursor::new(answers.as_bytes().to_vec());
        let mut output = Vec::new();
        let result = trash.restore(OsStr::new(name), &mut input, &mut output);
        (result, String::from_utf8(output).unwrap())
    }

    #[test]
    fn restore_moves_the_entry_back() {
        let dir = temp_dir("restore");
        let trash = trash_in(&dir);
        let original = dir.join("a.txt");
        add_entry(&trash, "a.txt", &original, "2026-09-24T22:00:00");

        let (result, output) = restore_with(&trash, "a.txt", "");

        assert_eq!(result.unwrap(), original);
        assert_eq!(fs::read_to_string(&original).unwrap(), "a.txt");
        assert!(!trash.files_dir().join("a.txt").exists());
        assert!(!trash.info_path(OsStr::new("a.txt")).exists());
        assert_eq!(output, "", "nothing to ask when the path is free");

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn restore_of_an_unknown_name_is_an_error() {
        let dir = temp_dir("restore-unknown");
        let trash = trash_in(&dir);

        let (result, _) = restore_with(&trash, "nope", "");

        assert!(matches!(result, Err(Error::NotInTrash(_))));

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn restore_into_a_missing_directory_is_an_error() {
        let dir = temp_dir("restore-missing-dir");
        let trash = trash_in(&dir);
        add_entry(
            &trash,
            "a.txt",
            &dir.join("gone").join("a.txt"),
            "2026-09-24T22:00:00",
        );

        let (result, _) = restore_with(&trash, "a.txt", "");

        assert!(matches!(result, Err(Error::RestoreDirMissing(_))));
        assert!(
            trash.files_dir().join("a.txt").exists(),
            "entry stays in the trash"
        );

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn restore_onto_an_existing_file_suggests_a_numbered_name() {
        let dir = temp_dir("restore-suggest");
        let trash = trash_in(&dir);
        let original = dir.join("a.txt");
        fs::write(&original, "newer").unwrap();
        add_entry(&trash, "a.txt", &original, "2026-09-24T22:00:00");

        // An empty answer accepts the suggestion.
        let (result, output) = restore_with(&trash, "a.txt", "\n");

        assert_eq!(result.unwrap(), dir.join("a.txt_1"));
        assert!(output.contains("[a.txt_1]"), "prompt was: {output:?}");
        assert_eq!(fs::read_to_string(&original).unwrap(), "newer");
        assert_eq!(fs::read_to_string(dir.join("a.txt_1")).unwrap(), "a.txt");

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn restore_suggestion_skips_names_already_taken() {
        let dir = temp_dir("restore-suggest-skip");
        let trash = trash_in(&dir);
        fs::write(dir.join("a.txt"), "").unwrap();
        fs::write(dir.join("a.txt_1"), "").unwrap();
        add_entry(&trash, "a.txt", &dir.join("a.txt"), "2026-09-24T22:00:00");

        let (result, _) = restore_with(&trash, "a.txt", "\n");

        assert_eq!(result.unwrap(), dir.join("a.txt_2"));

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn restore_onto_a_broken_symlink_still_prompts() {
        let dir = temp_dir("restore-onto-broken-symlink");
        let trash = trash_in(&dir);
        symlink(dir.join("nowhere"), dir.join("a.txt")).unwrap();
        add_entry(&trash, "a.txt", &dir.join("a.txt"), "2026-09-24T22:00:00");

        let (result, _) = restore_with(&trash, "a.txt", "\n");

        assert_eq!(result.unwrap(), dir.join("a.txt_1"));

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn restore_uses_the_name_typed_in() {
        let dir = temp_dir("restore-typed");
        let trash = trash_in(&dir);
        fs::write(dir.join("a.txt"), "").unwrap();
        add_entry(&trash, "a.txt", &dir.join("a.txt"), "2026-09-24T22:00:00");

        let (result, _) = restore_with(&trash, "a.txt", "b.txt\n");

        assert_eq!(result.unwrap(), dir.join("b.txt"));
        assert_eq!(fs::read_to_string(dir.join("b.txt")).unwrap(), "a.txt");

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn restore_asks_again_when_the_typed_name_is_taken() {
        let dir = temp_dir("restore-ask-again");
        let trash = trash_in(&dir);
        fs::write(dir.join("a.txt"), "").unwrap();
        fs::write(dir.join("taken.txt"), "keep me").unwrap();
        add_entry(&trash, "a.txt", &dir.join("a.txt"), "2026-09-24T22:00:00");

        let (result, _) = restore_with(&trash, "a.txt", "taken.txt\nfree.txt\n");

        assert_eq!(result.unwrap(), dir.join("free.txt"));
        assert_eq!(
            fs::read_to_string(dir.join("taken.txt")).unwrap(),
            "keep me"
        );

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn restore_aborts_when_input_ends() {
        let dir = temp_dir("restore-eof");
        let trash = trash_in(&dir);
        fs::write(dir.join("a.txt"), "").unwrap();
        add_entry(&trash, "a.txt", &dir.join("a.txt"), "2026-09-24T22:00:00");

        let (result, _) = restore_with(&trash, "a.txt", "");

        assert!(matches!(result, Err(Error::Aborted)));
        assert!(
            trash.files_dir().join("a.txt").exists(),
            "entry stays in the trash"
        );

        fs::remove_dir_all(&dir).unwrap();
    }
}
