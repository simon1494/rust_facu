#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Playlist {
    nombre: String,
    canciones: Vec<Cancion>,
}

#[allow(dead_code)]
impl Playlist {
    fn new(nombre: &str) -> Self {
        Playlist {
            nombre: nombre.to_string(),
            canciones: Vec::new(),
        }
    }

    pub fn agregar_cancion(&mut self, cancion: Cancion) {
        self.canciones.push(cancion);
    }

    pub fn eliminar_cancion(&mut self, titulo: &str) {
        self.canciones.retain(|c| c.titulo != titulo);
    }

    pub fn mover_cancion(&mut self, titulo: &str, nueva_pos: usize) {
        if let Some(indice) = self.canciones.iter().position(|c| c.titulo == titulo) {
            if nueva_pos < self.canciones.len() {
                let cancion = self.canciones.remove(indice);
                self.canciones.insert(nueva_pos, cancion);
            }
        }
    }

    pub fn buscar_cancion(&self, titulo: &str) -> Option<&Cancion> {
        self.canciones.iter().find(|c| c.titulo == titulo)
    }

    pub fn obtener_canciones_por_genero(&self, genero: Genero) -> Vec<&Cancion> {
        self.canciones
            .iter()
            .filter(|c| c.genero == genero)
            .collect()
    }

    pub fn obtener_canciones_por_artista(&self, artista: &str) -> Vec<&Cancion> {
        self.canciones
            .iter()
            .filter(|c| c.artista == artista)
            .collect()
    }

    pub fn modificar_titulo(&mut self, nuevo_nombre: &str) {
        self.nombre = nuevo_nombre.to_string();
    }

    pub fn eliminar_todas(&mut self) {
        self.canciones.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crear_cancion(titulo: &str, artista: &str, genero: Genero) -> Cancion {
        Cancion {
            titulo: titulo.to_string(),
            artista: artista.to_string(),
            genero,
        }
    }

    #[test]
    fn test_agregar_cancion() {
        let mut playlist = Playlist::new("asdf");
        let c = crear_cancion("Que se yo", "Tu vieja", Genero::Pop);
        playlist.agregar_cancion(c.clone());
        assert_eq!(playlist.canciones.len(), 1);
        assert_eq!(playlist.canciones[0], c);
    }

    #[test]
    fn test_eliminar_cancion_existente() {
        let mut playlist = Playlist::new("wiwi");
        playlist.agregar_cancion(crear_cancion("meh", "Dani agostini", Genero::Pop));
        playlist.eliminar_cancion("meh");
        assert!(playlist.canciones.is_empty());
    }

    #[test]
    fn test_eliminar_cancion_inexistente() {
        let mut playlist = Playlist::new("alooo");
        playlist.agregar_cancion(crear_cancion("Vieja", "Las pelotas", Genero::Pop));
        playlist.eliminar_cancion("no existe");
        assert_eq!(playlist.canciones.len(), 1);
    }

    #[test]
    fn test_mover_cancion_posicion_valida() {
        let mut playlist = Playlist::new("Armate una");
        playlist.agregar_cancion(crear_cancion("qwer", "Tu", Genero::Rock));
        playlist.agregar_cancion(crear_cancion("sdfg", "Vieja", Genero::Pop));
        playlist.agregar_cancion(crear_cancion("xcvb", "En tanga", Genero::Jazz));
        playlist.mover_cancion("xcvb", 0);
        assert_eq!(playlist.canciones[0].titulo, "xcvb");
    }

    #[test]
    fn test_mover_cancion_posicion_fuera_de_rango() {
        let mut playlist = Playlist::new("la polizia");
        playlist.agregar_cancion(crear_cancion("Roxanaaa", "The police", Genero::Jazz));
        playlist.mover_cancion("Roxanaaa", 5); // no hace nada
        assert_eq!(playlist.canciones[0].titulo, "Roxanaaa");
    }

    #[test]
    fn test_buscar_cancion_existente() {
        let mut playlist = Playlist::new("Terminaitor");
        playlist.agregar_cancion(crear_cancion("Hasta la vista", "Baby", Genero::Rock));
        let resultado = playlist.buscar_cancion("Hasta la vista");
        assert!(resultado.is_some());
        assert_eq!(resultado.unwrap().artista, "Baby");
    }

    #[test]
    fn test_buscar_cancion_inexistente() {
        let playlist = Playlist::new("Vacia");
        assert!(playlist.buscar_cancion(":O").is_none());
    }

    #[test]
    fn test_obtener_canciones_por_genero() {
        let mut playlist = Playlist::new("oaaa");
        playlist.agregar_cancion(crear_cancion("Cuello", "Los piojos", Genero::Pop));
        playlist.agregar_cancion(crear_cancion("Manos", "Mana", Genero::Pop));
        playlist.agregar_cancion(crear_cancion("Ojos", "Dolina", Genero::Rock));
        let pop = playlist.obtener_canciones_por_genero(Genero::Pop);
        assert_eq!(pop.len(), 2);
        assert!(pop.iter().all(|c| c.genero == Genero::Pop));
    }

    #[test]
    fn test_obtener_canciones_por_artista() {
        let mut playlist = Playlist::new("Mi Playlist");
        playlist.agregar_cancion(crear_cancion("Cuello", "Los piojos", Genero::Pop));
        playlist.agregar_cancion(crear_cancion("Manos", "Los piojos", Genero::Pop));
        playlist.agregar_cancion(crear_cancion("Ojos", "Dolina", Genero::Rock));
        let u2 = playlist.obtener_canciones_por_artista("Los piojos");
        assert_eq!(u2.len(), 2);
        assert!(u2.iter().all(|c| c.artista == "Los piojos"));
    }

    #[test]
    fn test_modificar_titulo() {
        let mut playlist = Playlist::new("tutan");
        playlist.modificar_titulo("kamon");
        assert_eq!(playlist.nombre, "kamon");
    }

    #[test]
    fn test_eliminar_todas() {
        let mut playlist = Playlist::new("asdf");
        playlist.agregar_cancion(crear_cancion("Cancion 1", "ewrtg", Genero::Rap));
        playlist.agregar_cancion(crear_cancion("Cancion 2", "wert", Genero::Jazz));
        playlist.eliminar_todas();
        assert!(playlist.canciones.is_empty());
    }
}
