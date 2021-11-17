pub fn read_lines_as_strings(filename: String) -> Vec<u32> {
    let txt =
        std::fs::read_to_string(&filename).expect(&format!("could not read filename {}", filename));

    txt.split("\n")
        .map(|line| {
            line.parse::<u32>()
                .expect(&format!("'{}' could not parse to u32", line))
        })
        .collect()
}
