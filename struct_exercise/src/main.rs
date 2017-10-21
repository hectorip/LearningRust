
struct Astro {
    nombre: String,
    orbita_sideral: i32
}

fn listar_astros() {
    let sistema_solar = [
        Astro { nombre: "Plutón".to_string(), orbita_sideral: 125 },
    ];
    for astro in &sistema_solar{
        println!("Astro: {0}, Órbita: {1}", astro.nombre, astro.orbita_sideral)
    }
}

fn main() {
    listar_astros();
}
