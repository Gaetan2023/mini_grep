////////////////////////////////////////////////////////
// le programme  cherche un mot dans un fichie        //
//  la ligne qui contient le mot                      //
///////////////////////////////////////////////////////
use std::env;  //importation du module qui permet de recuperer le mot et nom du fichier en ligne de command
use mini_grep::{Config,run};  
use std::process;//pour le signe du processus parent qui appelera le programme

fn main() {
    let args:Vec< String> = env::args().collect();//creer une variable du type vecteur string qui contient les valeur de 
                                                      // retour de l'iterateur args qui sont coolecter dans collect()
   
    //enregistrer les valeurs des argument dans les variables
    //unwrap_or_less donne Ok si tout va bien ou Err dans err si tout va mal
    //config fait l'extration d'argument en ligne de commande et les met dans dans structrure config
    let config= Config::new(&args).unwrap_or_else(|err|{
                          eprintln!("Probleme rencontrer lors de l'interpretation des arguments:{}",err);
                           process::exit(1);
                            });

       // run execute l'ouverture du fichier                     
     if let Err(e) = run(config) {
       eprintln!("Erreur applicative:{}",e);//sortie erreur
        process::exit(1);
     }
  

}

