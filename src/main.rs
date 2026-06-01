use std::env;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::Command;

fn install_package(package: String) -> Result<(), Box<dyn std::error::Error>> {
	let tmp_dir = env::temp_dir().join(&package);

	Command::new("clear").status()?;

	Command::new("git")
		.arg("clone")
		.arg(format!(
			"https://aur.archlinux.org/{}.git",
			package
		))
		.arg(&tmp_dir)
		.status()?;

	Command::new("less")
		.arg("PKGBUILD")
		.current_dir(&tmp_dir)
		.status()?;

	Command::new("makepkg")
		.arg("-si")
		.current_dir(&tmp_dir)
		.status()?;

	Ok(())
}

#[tokio::main]
async fn main() {
	let args_tmp: Vec<String> = env::args().collect();
	let args = &args_tmp[1..];

	for arg in args {
		let packages = aur_rpc::search(arg).await.unwrap();

		let mut count = 0;

		for package in &packages {
			println!("({}): {} - {} - {}\n    {}\n",
				count,
				package.id.clone(),
				package.name.clone(),
				package.maintainer.clone().unwrap_or_default(),
				package.description.clone().unwrap_or_default()
			);

			count += 1;
		}

		print!("Selection: ");
		let _ = stdout().flush();

		let mut s = String::new();
		stdin().read_line(&mut s).unwrap();

		let input: i32 = s.trim().parse::<i32>().unwrap();

		let _ = install_package(packages[input as usize].name.clone());
	}
}