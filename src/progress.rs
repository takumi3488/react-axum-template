#[macro_export]
macro_rules! bar {
    ($message:expr, $pos:expr) => {{
        let bar = ProgressBar::new_spinner()
            .with_message($message)
            .with_position($pos);
        bar.set_style(ProgressStyle::with_template("[{pos:>1}/9]{spinner} {msg}").unwrap());
        bar.enable_steady_tick(Duration::from_millis(100));
        bar
    }};
}
