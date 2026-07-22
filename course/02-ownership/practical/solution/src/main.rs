#[derive(Debug, PartialEq)]
struct DeliveryContact {
    customer: String,
    address: String,
}

fn normalize_text(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn format_label(contact: &DeliveryContact) -> String {
    format!("{}\n{}", contact.customer, contact.address)
}

fn rename_customer(contact: &mut DeliveryContact, new_name: &str) {
    contact.customer = normalize_text(new_name);
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
    fn normalizes_whitespace() {
        assert_eq!(normalize_text("  Amina   Nuru\n"), "Amina Nuru");
    }

    #[test]
    fn returns_borrowed_first_word() {
        assert_eq!(first_word("Amina Nuru"), "Amina");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn formats_without_consuming_contact() {
        let contact = contact();
        assert_eq!(
            format_label(&contact),
            "Amina Nuru\n101 Baobab Way, Nairobi"
        );
        assert_eq!(contact.customer, "Amina Nuru");
    }

    #[test]
    fn renames_through_mutable_borrow() {
        let mut contact = contact();
        rename_customer(&mut contact, "  Amina   K. Nuru ");
        assert_eq!(contact.customer, "Amina K. Nuru");
    }
}
