use crate::own_crates::es_primo::es_primo;
#[allow(dead_code)]
trait CheckPrime {
    fn is_prime(&self, number: i32) -> bool;
}
#[allow(dead_code)]
struct Numbers {
    integers: Vec<i32>,
}

impl CheckPrime for Numbers {
    fn is_prime(&self, number: i32) -> bool {
        if es_primo(number) {
            return true;
        }
        return false;
    }
}
#[allow(dead_code)]
impl Numbers {
    pub fn count_primes(&self) -> i32 {
        self.integers.iter().filter(|x| self.is_prime(**x)).count() as i32
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::own_crates::ejercicio_1::Numbers;

    #[test]
    fn test_ningun_numero_primo() {
        let vector = vec![4, 6, 8];
        let instancia = Numbers { integers: vector };
        assert_eq!(0, instancia.count_primes());
    }

    #[test]
    fn test_un_numero_primo() {
        let vector = vec![4, 5, 8];
        let instancia = Numbers { integers: vector };
        assert_eq!(1, instancia.count_primes());
    }

    #[test]
    fn test_varios_numeros_primos() {
        let vector = vec![5, 7, 11];
        let instancia = Numbers { integers: vector };
        assert_eq!(3, instancia.count_primes());
    }

    #[test]
    fn test_vector_vacio() {
        let vector = vec![];
        let instancia = Numbers { integers: vector };
        assert_eq!(0, instancia.count_primes());
    }
}
