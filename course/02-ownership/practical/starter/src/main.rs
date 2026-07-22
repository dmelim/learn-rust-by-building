#[derive(Debug, PartialEq)]
struct DeliveryContact {
    customer: String,
    address: String,
}

fn normalize_text(input: &str) -> String {
    // TODO: Collapse whitespace and return newly owned text.
    input.to_owned()
}

fn first_word(text: &str) -> &str {
    // TODO: Return a slice borrowed from text.
    let _ = text;
    ""
}

fn format_label(contact: &DeliveryContact) -> String {
    // TODO: Format both fields without consuming contact.
    let _ = contact;
    String::new()
}

fn rename_customer(contact: &mut DeliveryContact, new_name: &str) {
    // TODO: Update the owned name through the mutable borrow.
    let _ = (contact, new_name);
}

fn main() {
    let mut contact = DeliveryContact {
        customer: normalize_text("  Amina   Nuru "),
        address: normalize_text("  101 Baobab Way,   Nairobi "),
    };

    rename_customer(&mut contact, "Amina K. Nuru");
    println!("Label\n-----\n{}", format_label(&contact));
    println!("First name: {}", first_word(&contact.customer));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn contact() -> DeliveryContact {
        DeliveryContact {
            customer: "Amina Nuru".to_owned(),
            address: "101 Baobab Way, Nairobi".to_owned(),
        }
    }

    #[test]
    #[ignore = "implement whitespace normalization"]
    fn normalizes_whitespace() {
        assert_eq!(normalize_text("  Amina   Nuru\n"), "Amina Nuru");
    }

    #[test]
    #[ignore = "return the first word slice"]
    fn returns_borrowed_first_word() {
        assert_eq!(first_word("Amina Nuru"), "Amina");
        assert_eq!(first_word(""), "");
    }

    #[test]
    #[ignore = "format the borrowed contact"]
    fn formats_without_consuming_contact() {
        let contact = contact();
        assert_eq!(
            format_label(&contact),
            "Amina Nuru\n101 Baobab Way, Nairobi"
        );
        assert_eq!(contact.customer, "Amina Nuru");
    }

    #[test]
    #[ignore = "rename through a mutable borrow"]
    fn renames_through_mutable_borrow() {
        let mut contact = contact();
        rename_customer(&mut contact, "  Amina   K. Nuru ");
        assert_eq!(contact.customer, "Amina K. Nuru");
    }
}
