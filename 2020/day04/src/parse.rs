use crate::id::Id;

pub fn parse(data: &str) -> Vec<Id> {
    let mut result = Vec::new();

    for block in data.replace("\r", "").split("\n\n") {
        let mut ident = Id::new();
        let block = block.replace("\n", " ");
        for item in block.split(" ") {
            let item = item.trim();
            if !item.is_empty() {
                ident.set(item);
            }
        }
        result.push(ident);
    }

    result
}
