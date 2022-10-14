use std::process::Command;
use std::str;

use chrono::Utc;

fn main() {
    let docker = Command::new("docker").arg("--version").output().unwrap();
    if !docker.stderr.is_empty() {
        eprintln!("Error: `docker` is not installed");
        std::process::exit(1);
    }
    let pg = Command::new("psql").arg("--version").output().unwrap();
    if !pg.stderr.is_empty() {
        eprintln!("Error: `psql` is not installed");
        std::process::exit(1);
    }
    let sqlx = Command::new("sqlx").arg("--version").output().unwrap();
    if !sqlx.stderr.is_empty() {
        eprintln!("Error: `sqlx` is not installed");
        eprintln!(
            "HELP: please use `cargo install sqlx-cli --no-default-features --features postgres`"
        );
        std::process::exit(1);
    }

    let user = "postgres";
    let pass = "password";
    let db_name = "sandbox";
    let port = "5432";
    let host = "localhost";
    let db_url = format!("postgres://{user}:{pass}@localhost:{port}/{db_name}");
    println!("DATABASE_URL: {}", db_url);

    let running_container = Command::new("docker")
        .arg("ps")
        .args(["--filter", "name=postgres"])
        .args(["--format", "{{.ID}}"])
        .output();
    match running_container {
        Ok(o) => {
            if !o.stderr.is_empty() {
                eprintln!("{}", str::from_utf8(&o.stderr).unwrap());
                std::process::exit(1);
            }

            if !o.stdout.is_empty() {
                eprintln!("There is a postgres container already running.");
                eprintln!(
                    "Kill it with: `docker kill {}`",
                    str::from_utf8(&o.stdout).unwrap()
                );
                std::process::exit(1);
            }

            let date = Utc::now().format("%s").to_string();
            let r = Command::new("docker")
                .arg("run")
                .args(["-e", &format!("POSTGRES_USER={user}")])
                .args(["-e", &format!("POSTGRES_PASSWORD={pass}")])
                .args(["-e", &format!("POSTGRES_DB={db_name}")])
                .args(["-p", &format!("{port}:5432")])
                .arg("-d")
                .args(["--name", &format!("postgres_{date}")])
                .arg("postgres")
                .args(["-N", "1000"])
                .output()
                .unwrap();
            dbg!(&r);
            if !r.stderr.is_empty() {
                println!("{}", str::from_utf8(&o.stderr).unwrap());
                std::process::exit(1);
            }
        }
        Err(e) => todo!(),
    }

    println!("Postgres is up and running on port: {port}");
    let c = Command::new("sqlx")
        .arg("database")
        .arg("create")
        .args(["--database-url", &db_url])
        .output()
        .unwrap();
    let m = Command::new("sqlx")
        .arg("migrate")
        .arg("run")
        .args(["--database-url", &db_url])
        .output()
        .unwrap();
}
