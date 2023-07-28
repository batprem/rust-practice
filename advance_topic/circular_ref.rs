// student* --- * course

// students
// course
// Vec<Enrollment { course, student }>
use std::rc::Rc;
use std::cell::RefCell;



struct Student {
    name: String,
    courses: Vec<Rc<RefCell<Course>>>
}

impl Student {
    fn course(&self, platform: Platform) -> Vec<String> {
        platform.enrollments.iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

impl Student {
    fn new (name: &str) -> Student {
        Student {
            name: name.into(),
            courses: Vec::new()
        }
    }
}
struct Course {
    name: String,
    students: Vec<Rc<RefCell<Student>>>
}

impl Course {
    fn new(name: &str) -> Course {
        Course {
            name: name.into(),
            students: Vec::<Rc<RefCell<Student>>>::new()
        }
    }

    fn add_student(
        course: Rc<RefCell<Course>>,
        student: Rc<RefCell<Student>>
    ) {
        student.borrow_mut().courses.push(course.clone());
        course.borrow_mut().students.push(student);
    }
}

struct Enrollment<'a> {
    student: &'a Rc<RefCell<Student>>,
    course: &'a Course
}

impl <'a> Enrollment <'a> {
    fn new (student: &'a Rc<RefCell<Student>>, course: &'a Course) -> Enrollment<'a> {
        Enrollment {student, course}
    }
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>
}
impl<'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec ::new()
        }
    }
    fn enroll(
        &mut self,
        student: &'a Rc<RefCell<Student>>,
        course: &'a Course
    ){
        self.enrollments.push(
            Enrollment::new(student, course)
        );
    }
}
fn main() {
    let john = Rc::new(
        RefCell::new(
            Student::new("John")
        )
    );

    let jane = Rc::new(
        RefCell::new(
            Student::new("Jane")
        )
    );

    let course = Course::new("Rust Course");
    let magic_course = Rc::new(RefCell::new(course));
    // course.add_student(john); //Rc
    Course::add_student(magic_course.clone(), john).clone();
    Course::add_student(magic_course.clone(), jane.clone());

    let course = Course::new("Rust Course");
    let mut p = Platform::new();
    p.enroll(&john, &course);
}