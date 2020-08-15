
use rand::Rng;

fn main() {
    let player_choice= get_player_choice();
    let opponent_choice = get_opponent_choice();
    println!("{:?}",opponent_choice);
   
    let result = judge(player_choice, opponent_choice);

    if result == "引き分け"{
        println!("引き分け！もう一度！");
        play();
    }else{
        println!("あなたの{:?}ですう",result);
    }
   
}
fn play()->String {
    let player_choice= get_player_choice();
    let opponent_choice = get_opponent_choice();
    println!("{:?}",opponent_choice);
   
    return judge(player_choice, opponent_choice);

}
fn beats(choice:&String)-> std::string::String {

    match choice.as_str() {
        "グー" => "チョキ".to_string(),
        "パー" => "グー".to_string(),
        "チョキ" => "パー".to_string(),
        _       => panic!("failed to read line: {}")
    }
}
fn judge(player_choice:String,opponent_choice:String)->String{
    let (own_beats, other_beats) = (beats(&player_choice), beats(&opponent_choice));

    match (&*own_beats,  &*other_beats) {
        _ if own_beats == opponent_choice =>  "勝ち".to_string(),
        _ if other_beats == player_choice =>  "負け".to_string(),
        _                            => "引き分け".to_string()
    }
  
}
fn get_opponent_choice()->String{
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0, 2);
    let random_choices = ["グー","チョキ","パー"];
    return random_choices[random_index].to_string();
    
}
fn get_player_choice<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    return s.trim().parse().ok().unwrap();
}