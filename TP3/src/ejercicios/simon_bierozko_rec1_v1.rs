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
            if nueva_pos <= self.canciones.len() {
                let cancion = self.canciones.remove(indice);
                self.canciones
                    .insert(nueva_pos.min(self.canciones.len()), cancion);
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

#[allow(dead_code)]
impl Playlist {


    fn buscar_cancion_con_prefijo(&self, prefijo: &str) -> Option<Vec<Cancion>> {
        // Chequeo si el prefijo no esta vacio, en caso contrario, retorno None porque no se puede buscar nada
        if prefijo.is_empty(){
            return None
        }

        // Para homogeneizar las mayus y minus paso tanto el prefijo como los titulos de las canciones a lowercase.
        let prefijo_lower = prefijo.to_lowercase();
        let encontradas: Vec<Cancion> = self.canciones
            .iter()
            .filter(|c| c.titulo.to_lowercase().starts_with(&prefijo_lower))
            .cloned()
            .collect();

        // Si no hay encontradas, se retorna None
        if encontradas.is_empty() {
            return None
        }

        // Si hay matches, los retorno
        Some(encontradas)
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

    #[test]
    fn test_mover_cancion_a_ultima_posicion() {
        let mut playlist = Playlist::new("Extra");
        playlist.agregar_cancion(crear_cancion("A", "X", Genero::Rock));
        playlist.agregar_cancion(crear_cancion("B", "X", Genero::Rock));
        playlist.mover_cancion("A", 2);
        assert_eq!(playlist.canciones.last().unwrap().titulo, "A");
    }

    #[test]
    fn test_eliminar_cancion_con_titulos_duplicados() {
        let mut playlist = Playlist::new("Dup");
        playlist.agregar_cancion(crear_cancion("repe", "X", Genero::Rock));
        playlist.agregar_cancion(crear_cancion("repe", "Y", Genero::Pop));
        playlist.eliminar_cancion("repe");
        assert!(playlist.canciones.is_empty());
    }

    #[test]
    fn test_filtrar_sin_coincidencias() {
        let mut playlist = Playlist::new("nada");
        playlist.agregar_cancion(crear_cancion("cancion", "qsy", Genero::Jazz));
        assert!(
            playlist
                .obtener_canciones_por_genero(Genero::Pop)
                .is_empty()
        );
        assert!(playlist.obtener_canciones_por_artista("asdf").is_empty());
    }

        #[test]
    fn test_prefijo_con_una_coincidencia() {
        let mut playlist = Playlist::new("Prefijos");
        playlist.agregar_cancion(crear_cancion("Hola", "Artista", Genero::Pop));
        playlist.agregar_cancion(crear_cancion("Chau", "Otro", Genero::Rock));
        let res = playlist.buscar_cancion_con_prefijo("Ho").unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].titulo, "Hola");
    }

    #[test]
    fn test_prefijo_con_varias_coincidencias() {
        let mut playlist = Playlist::new("Prefijos");
        playlist.agregar_cancion(crear_cancion("Hola", "X", Genero::Pop));
        playlist.agregar_cancion(crear_cancion("Holanda", "Y", Genero::Pop));
        playlist.agregar_cancion(crear_cancion("Horno", "Z", Genero::Pop));
        let res = playlist.buscar_cancion_con_prefijo("Ho").unwrap();
        assert_eq!(res.len(), 3);
        assert!(res.iter().all(|c| c.titulo.starts_with("Ho")));
    }

    #[test]
    fn test_prefijo_inexistente() {
        let mut playlist = Playlist::new("Prefijos");
        playlist.agregar_cancion(crear_cancion("Hola", "X", Genero::Pop));
        assert!(playlist.buscar_cancion_con_prefijo("XYZ").is_none());
    }

    #[test]
    fn test_prefijo_vacio() {
        let mut playlist = Playlist::new("Prefijos");
        playlist.agregar_cancion(crear_cancion("Uno", "A", Genero::Pop));
        playlist.agregar_cancion(crear_cancion("Dos", "B", Genero::Rock));
        assert!(playlist.buscar_cancion_con_prefijo("").is_none());
    }


    #[test]
    fn test_prefijo_con_mayus() {
        let mut playlist = Playlist::new("Prefijos");
        playlist.agregar_cancion(crear_cancion("Hola", "X", Genero::Pop));
        let res = playlist.buscar_cancion_con_prefijo("HO").unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].titulo, "Hola");
    }

    #[test]
    fn test_prefijo_retorna_varias_con_mayus() {
        let mut playlist = Playlist::new("Prefijos");
        playlist.agregar_cancion(crear_cancion("hola mundo", "X", Genero::Pop));
        playlist.agregar_cancion(crear_cancion("Hoja seca", "Y", Genero::Pop));
        playlist.agregar_cancion(crear_cancion("HOLAmanola", "Z", Genero::Pop));
        let res = playlist.buscar_cancion_con_prefijo("ho").unwrap();
        assert_eq!(res.len(), 3);
    }

    #[test]
    fn test_playlist_vacia_retorna_none() {
        let playlist = Playlist::new("Vacia");
        assert!(playlist.buscar_cancion_con_prefijo("a").is_none());
    }

    #[test]
    fn test_prefijo_mas_largo_que_titulo() {
        let mut playlist = Playlist::new("Prefijos");
        playlist.agregar_cancion(crear_cancion("Sol", "X", Genero::Pop));
        assert!(playlist.buscar_cancion_con_prefijo("Solamente").is_none());
    }

    #[test]
    fn test_prefijo_igual_a_titulo() {
        let mut playlist = Playlist::new("Prefijos");
        playlist.agregar_cancion(crear_cancion("Sol", "X", Genero::Pop));
        let res = playlist.buscar_cancion_con_prefijo("Sol").unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].titulo, "Sol");
    }
}
