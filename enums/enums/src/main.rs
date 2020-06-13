#[derive(Debug)]

enum Student {
	Online,
	Onsite,
}

fn main() {
    let _student_online = Student::Online;
    let _student_onsite = Student::Onsite;

    println!("{:#?}", _student_online);
    println!("{:#?}", _student_onsite);

}
