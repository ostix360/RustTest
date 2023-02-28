use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
	let mut guess = String::new();
    println!("Tu veux jouer au devinette?");
	io::stdin()
			.read_line(&mut guess)
			.expect("Error during the user input");
	println!("Dans tout les cas tu n'as plus le choix!");
	io::stdin()
			.read_line(&mut guess)
			.expect("Error during the user input");
	println!("On va jouer a un jeu:
		Je vais imaginer un nombre et toi tu le devine, ok?");
	io::stdin()
			.read_line(&mut guess)
			.expect("Error during the user input");
	println!("Dans tout les cas je m'en fou.");
	io::stdin()
			.read_line(&mut guess)
			.expect("Error during the user input");
	println!("Je commence a partir de 0 (inclue) et je vais jusqu'a quel nombre?");
	
	let mut restart = true;
	
	while restart {
	
		let mut guess = String::new();
			
		io::stdin()
			.read_line(&mut guess)
			.expect("Error during the user input");
				
		let max: u32 = match guess.trim().parse() {
			Ok(nombre) => nombre,
			Err(_) => {
				println!("C'est quoi ton délire?");
				io::stdin()
					.read_line(&mut guess)
					.expect("Error during the user input");
				println!("Tu voulais que je crash tiens pour la peine je ne vais pas m'arreter!");
				io::stdin()
					.read_line(&mut guess)
					.expect("Error during the user input");
				loop {
					println!("Mouahahahahahahahahahahaha!!!")
				}
			}
		};
		
		
		
		let nombre_mystere = rand::thread_rng().gen_range(0..(max+1));
		
		// println!("Le nombre mystére est {}",  nombre_mystere);
		
		
		loop {
			println!("Choisie un nombre entre 0 et {}", max);
			let mut guess = String::new();
			
			io::stdin()
				.read_line(&mut guess)
				.expect("Error during the user input");
				
			let guess: u32 = match guess.trim().parse() {
				Ok(nombre) => nombre,
				Err(_) => continue
			};
			
			match guess.cmp(&nombre_mystere) {
				Ordering::Greater => println!("C'est moins"),
				Ordering::Less => println!("C'est plus"),
				Ordering::Equal => {
					println!("Bravo tu as trouvé");
					break;
				}
			}
		}
		io::stdin()
			.read_line(&mut guess)
			.expect("Error during the user input");
		println!("Bon par contre tu n'as rien gagné désolé :`(");
		io::stdin()
			.read_line(&mut guess)
			.expect("Error during the user input");
		println!("Mais c'étais marrant. on refait?");
		loop {
			println!("0 oui 1 non");
			let mut guess = String::new();
			
			io::stdin()
				.read_line(&mut guess)
				.expect("Error during the user input");
				
			let guess: u32 = match guess.trim().parse() {
				Ok(nombre) => nombre,
				Err(_) => {
					println!("Tu veux encore me faire crash hein fait trés attention...");
					continue;
				}
			};
			if guess == 0 {
				println!("Choisie le max!");
				break;
			}else if guess == 1{
				restart = false;
				break;
			}
		}
	}
	
	println!("C'étais sympa! Allez à la prochaine!");
	io::stdin()
			.read_line(&mut guess)
			.expect("Error during the user input");
}
