use pokers::member::{Dealer, Player};


fn main(){
    let mut d = Dealer::new();

    let mut players = Player::create(4);
    let p = &mut players;

    for each_player in p.iter_mut(){
        each_player.cards.push(d.draw_next());
    }
}

fn deal_to_players(){

}