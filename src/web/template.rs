use tera::Tera;

pub fn template() -> Tera {
    let mut tera = Tera::new("src/web/templates_new/**/*").unwrap();
    tera.full_reload().unwrap();

    tera
}
