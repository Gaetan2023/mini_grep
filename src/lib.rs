use std::fs; // manipulation des fichiers
use std::error::Error; //gestion de erreur
use std::env; //module contenant des fonctions pour la manipulations des variables env


pub struct  Config{
     pub recherche:String,
     pub nom_fichier:String,
     pub sensitive_case:bool
       }
impl Config {
//cette fonction permet de recuper les donnees collecter pour les mettre dans une structrure bien lier
      pub fn new(args: &[String]) -> Result<Config,&'static str> {
 //une bonne gestion d'erreur avec result qui aurra deux  args : config pour ok et err
 //verification de nombre d'argument
      if args.len() < 3{
     return  Err("il n'y a pas assez d'argumment");
                       }
 let recherche = args[1].clone();
 let nom_fichier = args[2].clone();
 let sensitive_case = env::var("INSENSIVE_CASE").is_err();//verifie si la vav env est present si
 //oui is_err retourne fasle et si non elle retourne true
Ok(Config { recherche, nom_fichier,sensitive_case }) 
}
}
 pub fn run(config:Config) -> Result<(),Box<dyn Error>>{

// lecture du fichier et affiche du contenu
let contenu = fs::read_to_string(config.nom_fichier)?;
let resultats;
if config.sensitive_case{ 
  resultats = search(&config.recherche, &contenu);
  } else {
   resultats =  check_insensive_casse(&config.recherche, &contenu);
  };

  for ligne in resultats {
    println!("{}", ligne);
    }
  Ok(())
}

//fonction de recherche
//qui aura un params generic de duree de vie 'a et
//un params mot_a_rechercher& , un params where_la_recherche _se_fera 
//dans un contenu avec une duree de vie 'a
//retour un vecteur qui contient la ligne contenant le_mot
pub fn search <'a>(recherche:&str,contenu:&'a str)->Vec<&'a str>//vecteur qui contiendra toutes les lignes contenant le mot
{
  let mut resultat =Vec::new();//creer un vecteur pour contenir la ligne contenat le mot
  for ligne in contenu.lines(){//iteration sur chaque ligne de contenu avec lines()
    
    if ligne.contains(recherche){ //verification si ligne contient le mot rechercher
    resultat.push(ligne);//mettre la ligne qui contient le mot
     }
    
 }
  resultat
}

pub fn check_insensive_casse<'a> (recherche:&str,contenu:&'a str) -> Vec<&'a str>{
  let mut resultat = Vec::new();
  let recherche = recherche.to_lowercase();
  for ligne in contenu.lines(){
    if ligne.to_lowercase().contains(&recherche){
      resultat.push(ligne);
    }
  }
  resultat
}
//ecriture test pour la conception d'une fonctionalite
#[cfg(test)]//test commence ici
mod test{
 

use super::*;
  #[test]
  //fonction de teste
  fn resultat() {
    let recherche ="ty";
    let contenu= "security,rapidity,productivity.
    obtenez les trois ensemble.";
    assert_eq!(vec!["security,rapidity,productivity."],search(recherche,contenu));
  }

  
  #[test]
  fn insence_casse(){
    let recherche = "RuSt";
    let contenu = "\
    Rust:
    sécurité, rapidité, productivité.
    Obtenez les trois en même temps.
    C'est pas rustique.";
    assert_eq!(vec!["Rust:","    C'est pas rustique."],check_insensive_casse(recherche,contenu));
  }
}
