use anyhow::{anyhow, bail, Context};

fn simple_anyhow() -> anyhow::Result<()> {
    bail!("Failed simple anyhow")
}

fn simple_anyhow_2() -> anyhow::Result<()> {
    Err(anyhow!("Failed simple anyhow 2"))
}

fn anyhow_context(input: &str) -> anyhow::Result<()> {
    let _: i32 = input.parse().context("Failed to parse value")?;
    Ok(())
}

fn anyhow_with_context() -> anyhow::Result<()> {
    let input = "xx";
    anyhow_context(input).with_context(|| format!("Anyhow with context {}", input))
}

fn main() {
    // println!("simple_anyhow {}", simple_anyhow().unwrap_err());
    // println!("simple_anyhow dbg {:?}", simple_anyhow().unwrap_err());
    // println!("simple_anyhow 2 {}", simple_anyhow_2().unwrap_err());

    // println!("Anyhow with context {}", anyhow_with_context().unwrap_err());
    //
    // println!(
    //     "Anyhow with context {}",
    //     anyhow_with_context().context("Failed in main").unwrap_err()
    // );
    //
    // println!(
    //     "Anyhow with context dbg {:?}",
    //     anyhow_with_context().context("Failed in main").unwrap_err()
    // );

    println!("Long example {:?}", long_example())
}

fn parse_item(value: &str) -> anyhow::Result<i32> {
    value
        .parse()
        .with_context(||format!("Failed to parse {}", value))
}

fn parse_items(items: &[(bool, &str)]) -> anyhow::Result<Vec<(bool, i32)>> {
    items
        .iter()
        .cloned()
        .map(|(attr, value)| Ok((attr, parse_item(value)?)))
        .collect::<anyhow::Result<Vec<(bool, i32)>>>()
        .context("Failed to parse items")
}

fn analyse_items(items: &[(bool, i32)]) -> anyhow::Result<()> {
    for (attr, value) in items {
        if *attr == (value % 2 == 0) {
            bail!("Analysis failed for {}, {}", attr, value)
        }
    }

    Ok(())
}

fn long_example() -> anyhow::Result<()> {
    let items = vec![(true, "1"), (true, "2"), (true, "3")];
    let parsed = parse_items(items.as_slice()).context("Failed to get parsed items")?;
    analyse_items(&parsed).context("Items analysis failed")
}