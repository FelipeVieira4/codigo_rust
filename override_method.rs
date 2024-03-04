// Feito com base em um exemplo catado da documentação Rust

pub struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl NewsArticle{
    fn new(hl: &str, lo:&str, au: &str, co: &str) -> NewsArticle{
        NewsArticle{
           headline: hl.to_string(),
           location: lo.to_string(),
           author: au.to_string(),
           content: co.to_string()
        }
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}


// Esse arrombadinho cria regras de funçõs
pub trait Summary {
    fn summarize(&self) -> String;
    fn quem_e_autor(&self) -> String;
	fn get_conteudo(&self) -> ();
}

// Implementação do Summary para o NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn quem_e_autor(&self) -> String{
        format!("{}",self.author)
    }
	 fn get_conteudo(&self) -> (){
        println!("O contéudo de Artigo é: {}",self.content);
    }
}

// Implementação do Summary para o Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn quem_e_autor(&self) -> String{
        format!("{}",self.username)
    }
	fn get_conteudo(&self) -> (){
        println!("O contéudo de Tweet é: {}",self.content);
    }
}

fn main(){
    let news_article = NewsArticle::new("Divindade do Java","1L 254","José","TEXTO");
    let tweet = Tweet{
        username: "joaoxx".to_string(),
        content: "queijo".to_string(),
        reply: true,
        retweet: false
    };
    
    println!("{}",news_article.summarize());
    news_article.get_conteudo();
    println!("{}", tweet.summarize());
    tweet.get_conteudo();
    
	println!("O autor do new article é: {}", news_article.quem_e_autor());
    println!("O autor do tweet é: {}", tweet.quem_e_autor());
	
	
	let news_article = NewsArticle::new("Divindade do Rust","1L 254","Matheus","TEXTO SOBRE RUST");
	println!("{}",news_article.summarize());
    news_article.get_conteudo();
	println!("O autor do new article é: {}", news_article.quem_e_autor());
}
    