
use  std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn verification_code(code: &str, code_valide : &str )  {
  let  code = "#119#";
  if code_valide != code {
    println!("veiller  entrez  un code valide pour continuer les transaction ");
  }
  
}
 
 fn main(){
     struct Forfait {

      forfait_appel : String,
      forfait_message: String,
      forfait_internet : String,
      forfait_tout_reseau:String,

     }
    println!("entrez le code pour  vos forfait  connexion ");
    let mut code = String::new();
    io::stdin().read_line(&mut code ).expect("erreur lors je le recuperation du code ");
    let   code_valide: &str = code.trim().to_string()
     println!("{:?}", code_valide);
      let code: &str = "#119#";

      let premier_forfait = "forfait_appel ";
      let deuxieme_forfait = "fofait_internet";
      let troisieme_forfait = "forfait_message";
      let dernier_forfait = "forfait_tout_reseaux";
      
      verification_code(code, code_valide);
      
    if code_valide  == code {

        println!("{:?}", premier_forfait);
        println!("{:?}",deuxieme_forfait );
        println!("{:?}", troisieme_forfait);
        println!("{:?}", dernier_forfait);
    }
  
    println!("entrez  le nom du forfait solicite ");
    let mut  choix = String::new();
    io::stdin().read_line(&mut choix ).expect("ereur  lors  de la recuperation de la requete ");
    let  forfait_de: String = choix.trim().parse().expect("erreur lors de la lecture ");
    println!("{:?}", forfait_de);
    let  forfait_desirer = forfait_de;

    if  forfait_desirer == premier_forfait {

      println!("{:?}", " 100 egale 2h d;appel");
      println!("{:?}", "1000 egale 2 jour d'appel");
      println!("{:?}" , "1000 ilimite  pour un moi ");

    }else if forfait_desirer == deuxieme_forfait {
      println!("{:?}" , "100 egale  a 100 MG");
      println!("{:?}" , "1000 egale 10 GG ");
      println!( "{:?}" , "10000 egalr  ilimite pour un moi ");

    }else if forfait_desirer == troisieme_forfait  {

      println!("{:?}" , "100 egale 100 message ");
      println!( "{:?}" , "1000 egale 1000 message ");
      println!("{:?}", "10000 egale ilimite ");

    }

  }
 



