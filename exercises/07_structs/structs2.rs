#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();

        // TODO: Create your own order using the update syntax and template above!
        // ADDED: Created a new order by defining some unique fields and performing a field-by-field copy from the template.
        // The copied field have no ownership of the template, they are just copies
        // to copy a field use: field_name: order_template.field_name
        let your_order = Order {
            name: "Hacker in Rust".to_string(),
            year: order_template.year,
            made_by_phone: order_template.made_by_phone,
            made_by_mobile: order_template.made_by_mobile,
            made_by_email: order_template.made_by_email,
            item_number: order_template.item_number,
            count: 1,
        };

        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);

        // Test that the template is still accessible and not modified.
        assert_eq!(order_template.name, "Bob");
        assert_eq!(order_template.year, 2019);
        assert_eq!(order_template.made_by_phone, false);
        assert_eq!(order_template.made_by_mobile, false);
        assert_eq!(order_template.made_by_email, true);
        assert_eq!(order_template.item_number, 123);
        assert_eq!(order_template.count, 0);


    }
}
