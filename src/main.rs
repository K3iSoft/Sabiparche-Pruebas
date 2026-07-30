pub fn mensaje() -> &'static str {
    "Sabiparche: proyecto público de pruebas operativo"
}

fn main() {
    println!("{}", mensaje());
}

#[cfg(test)]
mod tests {
    use super::mensaje;

    #[test]
    fn devuelve_el_mensaje_oficial() {
        assert_eq!(
            mensaje(),
            "Sabiparche: proyecto público de pruebas operativo"
        );
    }
}
