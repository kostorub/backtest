use tera::Tera;


pub fn template() -> Tera {
    let mut tera = Tera::new("src/web/html/*").unwrap();
    tera.full_reload().unwrap();

    tera.add_raw_template("select_options", 
    "{% for value in values %}
                <option value='{{ value }}'>{{ value }}</option>
            {% endfor %}").unwrap();

    tera.add_raw_template("metrics",
    r#"
    <table class="table">
        <thead>
            <tr>
            <th scope="col">Metric's name</th>
            <th scope="col">Value</th>
            </tr>
        </thead>
        <tbody>
            {% for key, value in values %}
                <tr>
                    <td>{{ key }}</td>
                    <td>{{ value }}</td>
                </tr>
            {% endfor %}
        </tbody>
        </table>
    "#
    ).unwrap();

    tera
}
