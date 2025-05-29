pub mod debug_console {
    use clap::{Command, FromArgMatches};
    use clap::ColorChoice::Auto;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

    pub async fn read_cli<Cmd>(prefix: &str, entry: String, cmd: Command) -> Option<Cmd>
    where
        Cmd: FromArgMatches,
    {
        let input: String = {
            let mut buffer = String::new();
            let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());
            let mut stdout = tokio::io::stdout();

            stdout.write_all(prefix.as_bytes()).await.ok().unwrap();
            stdout.flush().await.ok().unwrap();

            stdin.read_line(&mut buffer).await.ok().unwrap();
            buffer.trim().to_string()
        };

        process_debug_cli(entry, input, cmd).await
    }

    async fn process_debug_cli<Cmd>(entry: String, input: String, cmd: Command) -> Option<Cmd>
    where
        Cmd: FromArgMatches
    {
        if input.trim().is_empty() {
            return None;
        }

        let cmd = cmd
            .color(Auto)
            .help_template(
                "{subcommands}{options}"
            )
            .disable_help_flag(true)
            .disable_version_flag(true);

        let args = shell_words::split(input.as_str()).unwrap_or_else(|_e| {
            ["".to_string()].to_vec()
        });

        let full_args = std::iter::once(entry.into()).chain(args);

        match cmd.try_get_matches_from(full_args) {
            Ok(matches) => {
                match Cmd::from_arg_matches(&matches) {
                    Ok(cmd) => {
                        Some(cmd)
                    }
                    Err(_err) => {
                        None
                    }
                }
            }
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }
}