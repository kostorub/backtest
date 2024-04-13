use tera::Tera;

pub fn template() -> Tera {
    let mut tera = Tera::new("src/web/templates/**/*").unwrap();
    tera.full_reload().unwrap();

    tera
}
