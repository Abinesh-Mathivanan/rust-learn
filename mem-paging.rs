const PAGE_SIZE: usize = 4096;
const PAGE_TABLE_ENTRIES: usize = 1024;

#[derive(Debug)]
struct Page {
    data: [u8; PAGE_SIZE],
    is_present: bool,
}

#[derive(Debug)]
struct PageTable {
    entries: [Option<Page>; PAGE_TABLE_ENTRIES],
}

impl PageTable {
    fn new() -> Self {
        PageTable {
            entries: [None; PAGE_TABLE_ENTRIES],
        }
    }

    fn allocate_page(&mut self, page_number: usize) -> Result<&mut Page, &'static str> {
        if page_number >= PAGE_TABLE_ENTRIES {
            return Err("Page number out of bounds");
        }

        if self.entries[page_number].is_none() {
            self.entries[page_number] = Some(Page {
                data: [0; PAGE_SIZE],
                is_present: true,
            });
        }

        Ok(self.entries[page_number].as_mut().unwrap())
    }

    fn read_page(&self, page_number: usize) -> Result<&Page, &'static str> {
        if page_number >= PAGE_TABLE_ENTRIES {
            return Err("Page number out of bounds");
        }

        match &self.entries[page_number] {
            Some(page) if page.is_present => Ok(page),
            Some(_) => Err("Page is not present (swapped out)"),
            None => Err("Page does not exist"),
        }
    }
}

fn main() {
    let mut page_table = PageTable::new();

    match page_table.allocate_page(0) {
        Ok(page) => {
            page.data[0] = 42;
            println!("Page 0 allocated and written to");
        }
        Err(e) => println!("Error: {}", e),
    }

    match page_table.read_page(0) {
        Ok(page) => println!("Read value: {}", page.data[0]),
        Err(e) => println!("Error: {}", e),
    }
}
