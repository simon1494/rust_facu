#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum GeneroMusical {
    ROCK,
    POP,
    RAP,
    JAZZ,
    OTROS,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Cancion {
    titulo: String,
    artista: String,
    genero: GeneroMusical,
}

#[allow(dead_code)]
impl Cancion {
    pub fn new(titulo: String, artista: String, genero: GeneroMusical) -> Cancion {
        Cancion {
            titulo,
            artista,
            genero,
        }
    }
}

#[allow(dead_code)]
pub struct Playlist {
    titulo: String,
    canciones: Vec<Cancion>,
}

#[allow(dead_code)]
impl Playlist {
    pub fn new(titulo: String, canciones: Vec<Cancion>) -> Playlist {
        Playlist { titulo, canciones }
    }

    pub fn agregar_cancion(&mut self, cancion: Cancion) -> String {
        self.canciones.push(cancion.clone());
        return format!(
            "Cancion agregada --> {} - {}",
            cancion.titulo, cancion.artista
        );
    }

    pub fn eliminar_cancion(&mut self, cancion: Cancion) -> bool {
        if let Some(indice) = self.obtener_indice_cancion(cancion) {
            self.canciones.remove(indice);
            return true;
        }
        return false;
    }

    pub fn eliminar_todas(&mut self) -> bool {
        self.canciones.clear();
        return true;
    }

    fn obtener_indice_cancion(&self, cancion: Cancion) -> Option<usize> {
        if let Some(indice) = self
            .canciones
            .iter()
            .position(|x| x.titulo == cancion.titulo)
        {
            return Some(indice);
        }
        return None;
    }
}
