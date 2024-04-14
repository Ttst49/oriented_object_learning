use oriented_object_learning::Ticket;

fn ticket_process(){
    let mut ticket = Ticket::new();

    ticket.add_text("I ate a salad tonight");
    assert_eq!("",ticket.content());

    ticket.ask_verification();
    assert_eq!("",ticket.content());

    ticket.approve();
    assert_eq!("I ate a salad tonight",ticket.content());
}

fn main() {
    println!("Hello world!");
}
