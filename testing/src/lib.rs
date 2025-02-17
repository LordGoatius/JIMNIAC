#[cfg(test)]
pub mod test {
    #[test]
    fn testing_thread() {
        return;
        use std::io::{stdin, stdout, Write};
        use std::thread::{self, sleep};
        use std::time::Duration;
        use termion::event::Key;
        use termion::input::TermRead;
        use termion::raw::IntoRawMode;

        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        write!(
            stdout,
            "{}{}q to exit. Type stuff, use alt, and so on.{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        )
        .unwrap();
        stdout.flush().unwrap();

        let r = &mut stdout;

        let (tx, rx) = std::sync::mpsc::channel::<Option<Key>>();

        thread::scope(|s| {
            s.spawn(|| {
                for k in stdin.keys() {
                    match k.as_ref().unwrap() {
                        Key::Char('q') => {
                            tx.send(Some(Key::Char('q'))).unwrap();
                            break;
                        }
                        key => tx.send(Some(*key)).unwrap(),
                    }
                }
            });
            s.spawn(|| loop {
                sleep(Duration::from_secs(3));
                tx.send(None).unwrap();
            });
            s.spawn(move || loop {
                match rx.try_recv() {
                    Err(_) => continue,
                    Ok(Some(Key::Char('q'))) => break,
                    Ok(Some(Key::Ctrl('c'))) => break,
                    Ok(val) => {
                        print!("{val:?}");
                        let _ = r.flush();
                    }
                }
            });
        });

        write!(stdout, "{}", termion::cursor::Show).unwrap();
        println!("Exit\n");
    }
}
