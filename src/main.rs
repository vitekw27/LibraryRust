
#[derive(Debug)]
struct Book{
    id:i32,
    book_name:String,
    author_name:(String, String),
    genre:String,
    published:bool,
    
}

impl Book{
    pub fn new(id:i32, book_name:String, author_name:(String, String), genre:String) -> Book{

            Book {
                id :id,
                book_name: book_name,
                author_name:author_name,
                genre:genre,
                published:false,
               
            }
    }
    pub fn change_published(&mut self,state:bool){{
        self.published = state;
    }}
    pub fn format_writer(&self) -> String{
        format!("{} {}", self.author_name.0,self.author_name.1)
    }
}

struct Customer{
    id:i32,
    customer_name:(String, String),
    borrowed_books:Vec<Book>,
}

impl Customer{
    pub fn new(id:i32,name:(String,String))-> Customer{
        Customer{
            id:id,
            customer_name:name,
            borrowed_books: vec![],
        }
    }

    pub fn list_books(&self){
        for book in &self.borrowed_books {
            println!("{:?}",book);
        }
    }
}

struct Library{
    published_books:Vec<Book>,
    unpublished_books:Vec<Book>,
}

impl Library{

    pub fn new() -> Library{
        Library{
            published_books:vec![],
            unpublished_books:vec![],

        }
    }
    pub fn list_published_books(&self){
        for book in &self.published_books {
            println!("{:?}",book);
        }
    }
    pub fn list_unpublished_books(&self){
        for book in &self.unpublished_books {
            println!("{:?}",book);
        }
    }
    pub fn add_book(&mut self, book:Book){
        
        if book.published{
            &self.published_books.push(book);
        }else{
            &self.unpublished_books.push(book);
        }
    }
    pub fn borrow_book(&mut self,requested_book_name: String, customer:&mut Customer) -> Result<String,String>{

        for unpublished_book in &self.unpublished_books{
            if unpublished_book.book_name == requested_book_name{
                return Err(format!("Book {:?} is not published yet",&requested_book_name));
            }
        }

        for book in &self.published_books{
            if(book.book_name == requested_book_name){
                customer.borrowed_books.push(Book{
                    id:book.id,
                    book_name:book.book_name.clone(),
                    genre:book.genre.clone(),
                    author_name:book.author_name.clone(),
                    published:book.published.clone(),
                });
                 return Ok(format!("Book {:?} was succesfuly borrowed",&requested_book_name));
            }else {
               return Err(format!("Book {:?} not found",&requested_book_name));
            }
        }
        Ok("Book borrowed".into())
       
    }   
}


fn main() {
   let mut lib = Library::new();

    init_mockdata(&mut lib);
    let mut matysek = Customer::new(1,("Matej".into(),"Pycha".into()));
    match lib.borrow_book("The Wanderer".into(), &mut matysek){
        Ok(i) => println!("{}",i),
        Err(e) => println!("{}",e),
    }
    matysek.list_books();
}
pub fn init_mockdata(lib:&mut Library) {
    lib.add_book(Book { id: (1), book_name: ("Annabell".into()), author_name: ("JK".into(),"Rowling".into()), genre: ("Horror".into()), published: false });
    lib.add_book(Book { id: (2), book_name: ("The Wanderer".into()), author_name: ("George".into(), "Orwell".into()), genre: ("Science Fiction".into()), published: true });
    lib.add_book(Book { id: (3), book_name: ("Shadow Realm".into()), author_name: ("Brandon".into(), "Sanderson".into()), genre: ("Fantasy".into()), published: false });
    lib.add_book(Book { id: (4), book_name: ("Echoes of Silence".into()), author_name: ("Margaret".into(), "Atwood".into()), genre: ("Drama".into()), published: true });
    lib.add_book(Book { id: (5), book_name: ("Lunar Tides".into()), author_name: ("Ursula".into(), "K. Le Guin".into()), genre: ("Fantasy".into()), published: false });
    lib.add_book(Book { id: (6), book_name: ("Cybernetica".into()), author_name: ("Isaac".into(), "Asimov".into()), genre: ("Science Fiction".into()), published: true });
    lib.add_book(Book { id: (7), book_name: ("Beneath the Waves".into()), author_name: ("Ernest".into(), "Hemingway".into()), genre: ("Adventure".into()), published: false });
    lib.add_book(Book { id: (8), book_name: ("A Thousand Suns".into()), author_name: ("Khaled".into(), "Hosseini".into()), genre: ("Drama".into()), published: true });
    lib.add_book(Book { id: (9), book_name: ("The Timekeeper".into()), author_name: ("H.G.".into(), "Wells".into()), genre: ("Science Fiction".into()), published: false });
    lib.add_book(Book { id: (10), book_name: ("Threads of Destiny".into()), author_name: ("Jane".into(), "Austen".into()), genre: ("Romance".into()), published: true });
   
}
